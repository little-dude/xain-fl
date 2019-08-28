import time
from typing import List, Optional, Tuple

from absl import app, flags, logging

from xain.datasets import load_splits
from xain.helpers import storage

from . import report, run

FLAGS = flags.FLAGS

# Default parameters for _run_unitary_versus_federated
FLH_C = 0.1  # Fraction of participants used in each round of training
ROUNDS = 50  # Number of total rounds to train
FLH_E = 4  # Number of training epochs in each round
FLH_B = 64  # Batch size used by participants


"""
In this config the key in the dictionary will be the name of the benchmark
"""
benchmarks = {
    "integration_test": {
        "dataset_name": "fashion-mnist-100p-noniid-01cpp",
        "C": 0.02,  # two participants
        "E": 2,  # two epochs per round
        "rounds": 2,  # two rounds
    },
    "fashion-mnist-100p-iid-balanced": {
        "dataset_name": "fashion-mnist-100p-iid-balanced"
    },
    "fashion-mnist-100p-noniid-01cpp": {
        "dataset_name": "fashion-mnist-100p-noniid-01cpp"
    },
    "fashion-mnist-100p-noniid-02cpp": {
        "dataset_name": "fashion-mnist-100p-noniid-02cpp"
    },
    "fashion-mnist-100p-noniid-03cpp": {
        "dataset_name": "fashion-mnist-100p-noniid-03cpp"
    },
    "fashion-mnist-100p-noniid-04cpp": {
        "dataset_name": "fashion-mnist-100p-noniid-04cpp"
    },
    "fashion-mnist-100p-noniid-05cpp": {
        "dataset_name": "fashion-mnist-100p-noniid-05cpp"
    },
    "fashion-mnist-100p-noniid-06cpp": {
        "dataset_name": "fashion-mnist-100p-noniid-06cpp"
    },
    "fashion-mnist-100p-noniid-07cpp": {
        "dataset_name": "fashion-mnist-100p-noniid-07cpp"
    },
    "fashion-mnist-100p-noniid-08cpp": {
        "dataset_name": "fashion-mnist-100p-noniid-08cpp"
    },
    "fashion-mnist-100p-noniid-09cpp": {
        "dataset_name": "fashion-mnist-100p-noniid-09cpp"
    },
    "fashion-mnist-100p-noniid-10cpp": {
        "dataset_name": "fashion-mnist-100p-noniid-10cpp"
    },
}


def run_unitary_versus_federated(
    benchmark_name: str,
    dataset_name: str,
    C: float = FLH_C,
    E: int = FLH_E,
    B: int = FLH_B,
    rounds: int = ROUNDS,
):
    """
    :param C: Fraction of participants used in each round of training
    """
    logging.info(f"Starting {benchmark_name}")
    xy_splits, xy_val, xy_test = load_splits(dataset_name)

    start = time.time()

    # Train CNN on a single partition ("unitary learning")
    # TODO train n models on all partitions
    partition_id = 0
    logging.info(f"Run unitary training using partition {partition_id}")
    ul_hist, ul_loss, ul_acc = run.unitary_training(
        xy_splits[partition_id], xy_val, xy_test, epochs=rounds * E, batch_size=B
    )

    # Train CNN using federated learning on all partitions
    logging.info("Run federated learning using all partitions")
    fl_hist, _, fl_loss, fl_acc = run.federated_training(
        xy_splits, xy_val, xy_test, rounds, C=C, E=E, B=B
    )

    end = time.time()

    # Write results JSON
    results = {
        "name": benchmark_name,
        "start": start,
        "end": end,
        "duration": end - start,
        "FLH_C": C,
        "FLH_E": E,
        "FLH_B": B,
        "ROUNDS": rounds,
        "unitary_learning": {
            "loss": float(ul_loss),
            "acc": float(ul_acc),
            "hist": ul_hist,
        },
        "federated_learning": {
            "loss": float(fl_loss),
            "acc": float(fl_acc),
            "hist": fl_hist,
        },
    }
    storage.write_json(results, fname="results.json")

    # Plot results
    # TODO include aggregated participant histories in plot
    plot_data: List[Tuple[str, List[float], Optional[List[int]]]] = [
        (
            "Unitary Learning",
            ul_hist["val_acc"],
            [i for i in range(1, len(ul_hist["val_acc"]) + 1, 1)],
        ),
        (
            "Federated Learning",
            fl_hist["val_acc"],
            [i for i in range(E, len(fl_hist["val_acc"]) * E + 1, E)],
        ),
    ]
    # FIXME use different filenames for different datasets
    report.plot_accuracies(plot_data, fname="plot.png")


def main(_):
    benchmark_name = FLAGS.benchmark_name
    kwargs = benchmarks[benchmark_name]
    run_unitary_versus_federated(benchmark_name=benchmark_name, **kwargs)


if __name__ == "__main__":
    # Flags will be overriden by manually set flags as they will be parsed
    # again in the app.run invokation and overrides those set here
    FLAGS(["_", "--benchmark_name=fashion_mnist_100p_IID_balanced"])
    app.run(main=main)