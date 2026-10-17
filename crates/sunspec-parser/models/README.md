# Bundled SunSpec model definitions

These JSON files are every official SunSpec information model definition in
[github.com/sunspec/models](https://github.com/sunspec/models) (`json/`), commit
`90b4a331dcca1d6eac69c1bead952fddcc5852e0`, unmodified: 112 models. They are licensed under
the Apache License 2.0 (see `LICENSE` in this directory).

`build.rs` embeds every `model_<id>.json` in this directory into the binary, and
`src/registry.rs` parses them on first use to drive point decoding. To add or update a model,
copy `json/model_<id>.json` from that repository into this directory and update the commit
above; the tests check that every file here loads and decodes.

| Models | Description |
|---|---|
| 1 | Common (manufacturer, model, serial number) |
| 2–19 | Aggregator, secure dataset and certificate models, network and communication interfaces |
| 101–103 | Inverter, single/split/three phase (integer + scale factor) |
| 111–113 | Inverter, single/split/three phase (float) |
| 120–124 | Nameplate, basic settings, status, immediate controls, basic storage |
| 125–145 | IEEE 1547-2003 era controls: pricing, volt-var, freq-watt, ride-through curves, scheduling, extended settings |
| 160 | Multiple MPPT (repeating modules) |
| 201–204 | AC meters (integer + scale factor) |
| 211–214 | AC meters (float) |
| 220 | Secure AC meter selected readings |
| 302–308 | Environmental: irradiance, module temperature, inclinometer, GPS, reference point, met stations |
| 401–404 | String combiners |
| 501–502 | Solar modules |
| 601 | Tracker controller (draft) |
| 701–715 | IEEE 1547-2018 DER: measurements, capacity, enter service, controls, volt-var, volt-watt, trip curves, frequency droop, watt-var, storage capacity, DC measurement, DER control |
| 801–809 | Storage: energy storage base (deprecated), battery base, lithium-ion bank/string/module, flow battery bank/string/module/stack |
| 63001–63002 | SunSpec test models |
| 64001–64415 | Vendor and test-harness models in the official repository (Veris, Mersen, Eltek, OutBack, simulator and CSIP controls) |
