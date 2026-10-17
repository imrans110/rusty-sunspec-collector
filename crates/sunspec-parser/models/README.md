# Bundled SunSpec model definitions

These JSON files are the official SunSpec information model definitions from
[github.com/sunspec/models](https://github.com/sunspec/models), commit
`90b4a331dcca1d6eac69c1bead952fddcc5852e0`, unmodified. They are licensed under the
Apache License 2.0 (see `LICENSE` in this directory).

They are embedded into the binary at compile time (`src/registry.rs`) and drive point
decoding. To add a model, copy `json/model_<id>.json` from the same repository into
this directory and add it to `MODEL_SOURCES` in `src/registry.rs`.

| Models | Description |
|---|---|
| 1 | Common (manufacturer, model, serial number) |
| 101, 102, 103 | Inverter, single/split/three phase (integer + scale factor) |
| 111, 112, 113 | Inverter, single/split/three phase (float) |
| 120–124 | Nameplate, basic settings, status, controls, basic storage |
| 160 | Multiple MPPT (repeating modules) |
| 201–204 | AC meters |
| 701–704 | IEEE 1547-2018 DER: AC measurements, capacity, enter service, AC controls |
| 705–712 | IEEE 1547-2018 DER functions: volt-var, volt-watt, trip curves, frequency droop, watt-var |
| 713 | DER storage capacity |
| 802 | Battery |
