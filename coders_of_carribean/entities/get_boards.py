from pathlib import Path

import pandas as pd


def get_states():
    files = Path(".").glob("?.csv")
    return {fp.name: csv_to_pandas(fp) for fp in files}


def csv_to_pandas(fp):
    df = pd.read_csv(fp)
    df["coords"] = df["coords"].apply(eval)
    return df
