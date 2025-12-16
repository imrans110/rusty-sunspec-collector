# rusty-sunspec-collector plan

## Workspace
- Cargo workspace with crates: `collector-app` (binary orchestrator), `modbus-client`, `sunspec-parser`, `poller-actor`, `avro-kafka`, `discovery`, `types`.
- Keep `types` lightweight (DTOs/traits only); avoid heavy dependencies to prevent recompiling the world.

## Milestones
- Week 0: finalize workspace deps, fmt/clippy/test aliases, add sample configs.
- Week 1: `modbus-client` with connect/read helpers, per-request timeout/retry/backoff, quirks config (`max_batch_size`, optional inter-read delay), integration test against `diagslave`.
- Week 2: `sunspec-parser` that loads official JSON/XML, maps register addresses, implements scale-factor application, handles “Not Implemented” sentinels, lenient block-length handling, caches parsed models; golden tests using sample model files.
- Week 3: `poller-actor` + orchestrator using `JoinSet` for supervision; jittered poll loop with bounded channels; graceful shutdown and metrics on loop lag/timeouts.
- Week 4: `avro-kafka` with Avro schemas, producer wrapper (mockable), end-to-end harness from simulator to serialized payloads; `discovery` subnet scanner with concurrency cap and per-host timeout, plus `static_config` mode.

## Risks / mitigations
- Device variability: support `max_batch_size`, lenient reads for short/quirky models, retry narrow reads when full-block fails.
- SunSpec sentinels: treat `0x8000`/`NaN` etc. as `None` before scaling; return `Option<f64>`/`Result` from parsers.
- Actor failures: use supervision (e.g., `JoinSet`) to respawn failed polling tasks.
- Network sensitivity: allow discovery to be bypassed via static device lists.
