#!/usr/bin/env -S uv run -s --group build

"""Script to update the dnb subject categories."""

import sys
import unicodedata
from argparse import ArgumentParser
from dataclasses import asdict, dataclass
from pathlib import Path
from urllib.request import urlopen

import polars as pl
import pyoxigraph


@dataclass
class Concept:
    uri: str
    notation: str = ""
    label_de: str = ""
    label_en: str = ""


VOCAB_URI: str = "https://d-nb.info/standards/vocab/gnd/gnd-sc.ttl"
SC_PREFIX: str = "https://d-nb.info/standards/vocab/gnd/gnd-sc#"
SKOS_CONCEPT: str = "http://www.w3.org/2004/02/skos/core#Concept"
SKOS_NOTATION: str = "http://www.w3.org/2004/02/skos/core#notation"


def main() -> None:
    parser = ArgumentParser()
    parser.add_argument("--output", "-o", type=Path, required=True)
    args = parser.parse_args()

    data = urlopen(VOCAB_URI, timeout=15).read().decode("utf-8")

    # The vocabularies provided by the toolkit are always delivered in NFC
    # normal form.
    if not unicodedata.is_normalized("NFC", data):
        sys.exit(f"data load from {VOCAB_URI} is not NFC encoded!")

    graph = pyoxigraph.parse(data, pyoxigraph.RdfFormat.TURTLE)
    concepts: dict[str, Concept] = {}

    for s, p, o, _ in graph:
        uri = s.value

        if not uri.startswith(SC_PREFIX):
            continue

        if uri == SC_PREFIX or uri.endswith("*"):
            continue

        if uri not in concepts:
            concepts[uri] = Concept(uri)

        match p.value:
            case "http://www.w3.org/2004/02/skos/core#notation":
                concepts[uri].notation = str(o.value)
            case "http://www.w3.org/2004/02/skos/core#prefLabel":
                match o.language:
                    case "de":
                        concepts[uri].label_de = str(o.value)
                    case "en":
                        concepts[uri].label_en = str(o.value)

    df = pl.DataFrame([asdict(c) for c in concepts.values()]).sort("notation")

    # A notation must only appear once in the entire dataset.
    if df["notation"].n_unique() != df["notation"].len():
        sys.exit("At least one notation appears more than once!")

    df.write_csv(args.output, compression="gzip", compression_level=9)


if __name__ == "__main__":
    main()
