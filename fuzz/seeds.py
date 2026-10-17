#!/usr/bin/env python3
"""Writes seed corpora for the fuzz targets from the repository's fixtures.

Usage: fuzz/seeds.py OUT_DIR   (writes OUT_DIR/<target>/<seed> files)

- model_chain: each probe dump in fixtures/devices/ as a whole register map.
- decode_model: each model of each dump, prefixed with its model ID.
- config: docs/config.example.toml without its `*_file` lines (the target skips those).
"""

import json
import pathlib
import struct
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
STANDARD_BASES = [40000, 0, 50000]


def registers(text):
    return [int(word, 16) for word in text.split()]


def pack(words):
    return struct.pack(f">{len(words)}H", *words)


def main():
    out = pathlib.Path(sys.argv[1])
    for target in ("model_chain", "decode_model", "config"):
        (out / target).mkdir(parents=True, exist_ok=True)

    for path in sorted((ROOT / "fixtures" / "devices").glob("*.json")):
        dump = json.loads(path.read_text())
        base = dump["base_address"]
        # Bits 0-1 pick the map's base, bit 2 adds the "SunS" marker (see model_chain.rs).
        flags = (STANDARD_BASES.index(base) + 1 if base in STANDARD_BASES else 0) | 0b100
        chain = []
        for model in dump["models"]:
            words = registers(model["registers"])
            chain += words
            (out / "decode_model" / f"{path.stem}-{model['id']}").write_bytes(
                pack([model["id"]] + words)
            )
        chain += [0xFFFF, 0]
        (out / "model_chain" / path.stem).write_bytes(
            bytes([flags]) + pack([base]) + pack(chain)
        )

    example = (ROOT / "docs" / "config.example.toml").read_text().splitlines(keepends=True)
    (out / "config" / "config.example.toml").write_text(
        "".join(line for line in example if "_file" not in line)
    )


if __name__ == "__main__":
    main()
