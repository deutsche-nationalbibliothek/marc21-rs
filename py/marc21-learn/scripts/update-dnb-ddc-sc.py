#!/usr/bin/env -S uv run -s --group build

import pyoxigraph
from urllib.request import urlopen
from dataclasses import dataclass, asdict
import unicodedata

import polars as pl


@dataclass
class Concept:
    uri: str
    notation: str = ""
    label_de: str = ""
    label_en: str = ""
    note_de: str = ""
    note_en: str = ""


URI: str = "https://d-nb.info/standards/classification/ddc-sc.ttl"
SG_PREFIX: str = "https://d-nb.info/standards/classification/ddc-sc#SG"
SKOS_CONCEPT: str = "http://www.w3.org/2004/02/skos/core#Concept"
SKOS_NOTATION: str = "http://www.w3.org/2004/02/skos/core#notation"


def main() -> None:

    data = urlopen(URI, timeout=15).read().decode("utf-8")
    assert unicodedata.is_normalized("NFC", data)

    graph = pyoxigraph.parse(data, pyoxigraph.RdfFormat.TURTLE)
    concepts: dict[str, Concept] = {}

    for s, p, o, _ in graph:
        uri = s.value

        if not uri.startswith(SG_PREFIX):
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
            case "http://www.w3.org/2004/02/skos/core#note":
                match o.language:
                    case "de":
                        concepts[uri].note_de = str(o.value)
                    case "en":
                        concepts[uri].note_en = str(o.value)

    df = pl.DataFrame([asdict(c) for c in concepts.values()])
    assert df["notation"].n_unique() == df["notation"].len()

    df = df.with_columns(
        pl.col("notation").cast(pl.Enum(df["notation"].sort()))
    ).sort("notation")

    df.write_ipc("data/dnb-ddc-sc.ipc", compression="zstd")


if __name__ == "__main__":
    main()
