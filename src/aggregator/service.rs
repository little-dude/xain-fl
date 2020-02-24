use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use crate::{
    aggregator::rpc,
    common::{ClientId, Token},
    coordinator,
};

use tokio::{
    net::ToSocketAddrs,
    sync::{mpsc, oneshot},
};

use futures::{ready, stream::Stream};

/// A future that orchestrates the entire aggregator service.
// TODO: maybe add a HashSet or HashMap of clients who already
// uploaded their weights to prevent a client from uploading weights
// multiple times. Or we could just remove that ID from the
// `allowed_ids` map.

// TODO: maybe add a HashSet for clients that are already
// downloading/uploading, to prevent DoS attacks.
pub struct AggregatorService<A>
where
    A: Aggregator,
{
    /// Clients that the coordinator selected for the current
    /// round. They can use their unique token to download the global
    /// weights and upload their own local results once they finished
    /// training.
    allowed_ids: HashMap<ClientId, Token>,

    /// The latest global weights as computed by the aggregator.
    // NOTE: We could store this directly in the task that handles the
    // HTTP requests. I initially though that having it here would
    // make it easier to bypass the HTTP layer, which is convenient
    // for testing because we can simulate client with just
    // AggregatorHandles. But maybe that's just another layer of
    // complexity that is not worth it.
    global_weights: Arc<Vec<u8>>,

    /// The aggregator itself, which handles the weights or performs
    /// the aggregations.
    aggregator: A,

    /// A client for the coordinator RPC service.
    coordinator_rpc: Option<coordinator::rpc::Client>,

    /// A future that resolved to an RPC client. If is only necessary
    /// to poll it when the aggregator starts, until the first
    /// connection is established. After that, `coordinator_rpc` is
    /// set and the client automatically attempts to reconnect if the
    /// connection goes down.
    coordinator_rpc_connection: Option<coordinator::rpc::ConnectFuture>,

    /// RPC requests from the coordinator.
    rpc_requests: rpc::RequestReceiver,
    // http_requests: aggregator::http::Handle,
    aggregation_future: Option<AggregationFuture<A>>,
}

// FIXME: the futures returned by the `aggregate` method needs to be
// stored but it's not 'static since if take `&mut self`. For now we
// work around this by requiring + Clone + Send + Sync + 'static on
// the aggregator trait but that doens't seem like a good solution.
#[async_trait]
/// This trait defines the methods that an aggregator should
/// implement.
pub trait Aggregator {
    // FIXME: we should obviously require the Error bound, but for now
    // it's convenient to be able to use () as error type
    type Error;
    type AggregateFut: Future<Output = Result<Vec<u8>, Self::Error>> + Unpin;
    // type Error: Error;

    /// Check the validity of the given weights and if they are valid,
    /// add them to the set of weights to aggregate.
    async fn add_weights(&mut self, weights: Vec<u8>) -> Result<(), Self::Error>;

    /// Run the aggregator and return the result.
    fn aggregate(&mut self) -> Self::AggregateFut;
}

impl<A> AggregatorService<A>
where
    A: Aggregator,
{
    pub fn new<
        T: Clone + ToSocketAddrs + Send + Sync + 'static + Unpin,
        U: ToSocketAddrs + Send + Sync + 'static,
    >(
        aggregator: A,
        rpc_listen_addr: U,
        coordinator_rpc_addr: T,
    ) -> Self {
        let rpc_requests = rpc::run(rpc_listen_addr);
        Self {
            aggregator,
            rpc_requests,
            coordinator_rpc: None,
            coordinator_rpc_connection: Some(coordinator::rpc::ConnectFuture::new(
                coordinator_rpc_addr,
            )),
            allowed_ids: HashMap::new(),
            global_weights: Arc::new(vec![]),
            aggregation_future: None,
        }
    }

    fn poll_rpc_requests(&mut self, cx: &mut Context) -> Poll<()> {
        trace!("polling RPC requests");

        let mut stream = Pin::new(&mut self.rpc_requests);
        loop {
            match ready!(stream.as_mut().poll_next(cx)) {
                Some(rpc::Request::Select(((id, token), resp_tx))) => {
                    info!("handling rpc request: select {}", id);
                    self.allowed_ids.insert(id, token);
                    if resp_tx.send(()).is_err() {
                        warn!("RPC connection shut down, cannot send response back");
                    }
                }
                Some(rpc::Request::Aggregate(resp_tx)) => {
                    info!("handling rpc request: aggregate");
                    // reset the known IDs.
                    self.allowed_ids = HashMap::new();

                    self.aggregation_future = Some(AggregationFuture {
                        future: self.aggregator.aggregate(),
                        response_tx: resp_tx,
                    });
                }
                // The coordinator client disconnected. If the
                // coordinator reconnect to the RPC server, a new
                // AggregatorRpcHandle will be forwarded to us.
                None => {
                    warn!("RPC server shut down");
                    return Poll::Ready(());
                }
            }
        }
    }

    fn poll_aggregation(&mut self, cx: &mut Context) -> Poll<()> {
        if let Some(AggregationFuture {
            mut future,
            response_tx,
        }) = self.aggregation_future.take()
        {
            match Pin::new(&mut future).poll(cx) {
                Poll::Ready(Ok(weights)) => {
                    self.global_weights = Arc::new(weights);
                    if response_tx.send(()).is_err() {
                        error!("failed to send aggregation response to RPC task: receiver dropped");
                    }
                    Poll::Ready(())
                }
                Poll::Ready(Err(_)) => {
                    // no need to send a response. By dropping the
                    // `response_tx` channel, the RPC task will send
                    // an error.
                    error!("aggregation failed");
                    Poll::Ready(())
                }
                Poll::Pending => {
                    self.aggregation_future = Some(AggregationFuture {
                        future,
                        response_tx,
                    });
                    Poll::Pending
                }
            }
        } else {
            Poll::Pending
        }
    }
}

struct AggregationFuture<A>
where
    A: Aggregator,
{
    future: A::AggregateFut,
    response_tx: oneshot::Sender<()>,
}

impl<A> Future for AggregatorService<A>
where
    A: Aggregator + Unpin,
{
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        trace!("polling AggregatorService");
        let pin = self.get_mut();

        // This only runs when the aggregator starts
        if let Some(ref mut connection) = pin.coordinator_rpc_connection {
            match Pin::new(connection).poll(cx) {
                Poll::Ready(Ok(client)) => {
                    pin.coordinator_rpc = Some(client);
                    pin.coordinator_rpc_connection = None;
                }
                Poll::Ready(Err(e)) => {
                    error!("failed to connect RPC client: {}", e);
                    return Poll::Ready(());
                }
                _ => {}
            }
        }

        if let Poll::Ready(_) = pin.poll_rpc_requests(cx) {
            return Poll::Ready(());
        }

        Poll::Pending
    }
}

struct Credentials(ClientId, Token);

struct AggregatorServiceHandle {
    local_weights: mpsc::UnboundedSender<(Credentials, Vec<u8>)>,
    global_weights: mpsc::UnboundedSender<(Credentials, oneshot::Sender<Vec<u8>>)>,
}

impl AggregatorServiceHandle {
    async fn get_global_weights(&self, id: ClientId, token: Token) -> Option<Vec<u8>> {
        let (tx, rx) = oneshot::channel();
        if self
            .global_weights
            .clone()
            .send((Credentials(id, token), tx))
            .is_err()
        {
            return None;
        }
        rx.await.ok()
    }

    async fn set_local_weights(&self, id: ClientId, token: Token, weights: Vec<u8>) {
        let _ = self
            .local_weights
            .clone()
            .send((Credentials(id, token), weights));
    }
}
