# Code Review & Audit Report

This page records two reviews. The **October 2026 audit** is current. The **February 2026
review** below it is kept for history. Its fixes were committed without CI being green, and
`main` did not compile again until #9, so treat its "resolved" status with care.

---

## October 2026 audit

> **Scope:** the whole system: code, data path, data contract, configuration, observability,
> deployment, CI and docs.
> **Method:** code reading, plus running the collector against a SunSpec Modbus simulator to
> reproduce each failure. Every fix comes with a test that reproduces the original problem.
> **Status:** all 17 findings resolved in #9–#17.

Before the audit, `main` did not compile, and CI had been red since February 2026 without
anyone noticing (there was no branch protection). #9 restored the build and fixed the first
runtime bugs. #10 cleared all `cargo audit` vulnerabilities. The 8 Dependabot PRs were then
merged. The 17 findings below were fixed in #11–#17.

### Critical: blocked the core use case

| # | Finding | Fix |
|---|---|---|
| 1 | With any broker configured, Kafka publishing failed at startup: the default `zstd` compression wasn't compiled into librdkafka. | #11: rdkafka `zstd` feature, plus a regression test that creates a producer for every codec |
| 2 | Gateways: pollers were keyed by IP, so only the last unit ID behind a host was polled. | #12: devices keyed by host and unit ID, plus an end-to-end gateway test |
| 3 | Discovery read a fixed 200-register window and silently dropped every model past it (status, controls, MPPT). | #12: walk the model chain one header at a time, probing bases 40000, 0 and 50000 |
| 4 | A device that was offline at startup was never polled. | #12: per-device supervision with backoff, plus subnet rescans |
| 5 | SIGTERM (systemd/Docker stop) killed the process with no graceful shutdown. | #11: SIGTERM handled, plus a process-level test |

### High: data path and durability

| # | Finding | Fix |
|---|---|---|
| 6 | Upload throughput was capped at `batch_size / drain_interval` (200 samples/s). | #13: drain back to back while a backlog exists |
| 7 | The buffer was unbounded, so samples were lost once the disk filled; queued samples were lost on stop; every sample cost one fsync. | #13: `max_rows`/`max_age_s` retention, shutdown flush with a bounded final drain, batched transactions |
| 8 | Only raw registers were shipped. Scale factors, sentinels and point names were left to every consumer; the parser's scaling code was unused. | #14: edge decoding with the official SunSpec definitions |
| 9 | Each Kafka message was an Avro container mixing devices, with no key, the schema embedded every time, and duplicates after a crash. | #14: one keyed single-object record per sample; only acknowledged rows are deleted |

### Medium: operations and config

| # | Finding | Fix |
|---|---|---|
| 10 | Dropped Modbus connections were never re-established, so data was lost until the poller gave up. | #11: lazy reconnect on transport errors and timeouts; Modbus exceptions keep the connection |
| 11 | Config errors were silently ignored: unparsable env values, a bad unit ID became 1, unknown keys. | #15: strict config (`deny_unknown_fields`, errors name the variable); #17 deprecates the ineffective `modbus.timeout_ms` |
| 12 | About 430k INFO lines a day per device, and ANSI colour codes in journald/Docker logs. | #12 and #15: per-cycle logs at debug, colours only on a terminal, JSON format option |
| 13 | Metrics couldn't show device health: a high-cardinality `batch_size` label, summaries instead of histograms, no per-device liveness, and no real stall detection. | #12 and #15: `sunspec_*` metrics with per-device gauges, histograms, `/healthz`, a health-gated watchdog, stall detection |
| 14 | docker-compose couldn't pull its Kafka image and had no device to poll; the systemd unit ran as root; images were never built in CI; unused arm64 packages. | #16: official Kafka image, simulator service, DynamicUser unit, Docker E2E and cross-compile CI |

### Quality and process

| # | Finding | Fix |
|---|---|---|
| 15 | 13 tests in total; discovery, the poller and the orchestration had none; the integration tests passed without doing anything. | #11–#16: the `sunspec-sim` crate and simulator-backed unit, integration and end-to-end tests (56 plus 2 broker tests run in CI) |
| 16 | No branch protection, so a broken `main` went unnoticed for 8 months. | Branch protection requires Check & Lint, Test, Security Audit and Docker E2E |
| 17 | Docs overclaimed ("zero data loss", "90%+", "zombie poller detection", graceful SIGTERM) and linked to missing pages. | #17: docs rewritten against the code; `build.md` and `buffer_maintenance.md` added |

### Remaining limitations

- Kafka authentication is limited to SASL PLAIN and SCRAM (TLS, mutual TLS and SASL were added after the audit).
- There's no Schema Registry integration. Records carry the schema's fingerprint instead.
- Delivery is at-least-once. Consumers should deduplicate on `(device_key, model_id, collected_at_ms)`.
- There's no config reload and no per-device polling profiles.

---

## February 2026 review (historical)


### Executive Summary

Rusty SunSpec Collector is a well-architected Rust workspace with clean domain boundaries across 8 crates. The core design — actor-based polling, durable SQLite buffering, Avro/Kafka uplink — is sound.

The initial review uncovered **3 compilation/runtime bugs**, **8 high-severity issues**, and **10 medium-severity issues**. All have been addressed in the accompanying commits.

---

### 1. Critical Bugs — RESOLVED

#### 1.1 `config.rs` — Non-existent field `discovery_unit_ids` ✅

**Problem**: The `Default` impl, `apply_env_overrides`, and `apply_file_config` all referenced a `discovery_unit_ids` field that did not exist on `CollectorConfig`. Additionally, `parse_unit_id_list()` was called but never defined, and `FileDiscoveryConfig` was missing a `unit_ids` field.

**Fix**: Added `discovery_unit_ids: Vec<u8>` to the struct, added `unit_ids` to `FileDiscoveryConfig`, and implemented the missing `parse_unit_id_list()` function.

#### 1.2 `config.rs` — Duplicate fields in `Default` impl ✅

**Problem**: `kafka_topic` and `kafka_enable_idempotence` were set twice in the struct literal.

**Fix**: Removed the duplicate lines.

#### 1.3 `main.rs` — Prometheus recorder installed twice ✅

**Problem**: `PrometheusBuilder::new().install_recorder()` was called twice. The `metrics` crate only allows a single global recorder, so the second call would crash the app on startup.

**Fix**: Removed the duplicate builder + install.

---

### 2. High-Severity Issues — RESOLVED

#### 2.1 No CI/CD for build, test, or lint ✅

**Fix**: Added `.github/workflows/ci.yml` with `cargo fmt --check`, `cargo clippy`, `cargo build`, and `cargo test` jobs.

#### 2.2 Docker container runs as root ✅

**Fix**: Added non-root `collector` user/group in Dockerfile. Process now runs as `USER collector:collector`.

#### 2.3 No Docker `HEALTHCHECK` ✅

**Fix**: Added `HEALTHCHECK` instruction that curls the `/metrics` endpoint.

#### 2.4 Outdated base images ✅

**Fix**: Migrated from `bullseye` (Debian 11) to `bookworm` (Debian 12) for both builder and runtime stages. Updated `libssl1.1` to `libssl3`.

#### 2.5 `.dockerignore` excludes `Cargo.lock` ✅

**Fix**: Removed `Cargo.lock` from `.dockerignore`. `--locked` builds now work correctly in Docker.

#### 2.6 `.gitignore` excludes `Cargo.lock` ✅

**Fix**: Removed `Cargo.lock` from `.gitignore`. Added missing patterns: `.DS_Store`, `.env`, `*.sqlite*`, `.vscode/`, `.idea/`, editor swap files.

#### 2.7 `discovery.rs` — `expect()` on Semaphore can panic ✅

**Fix**: Replaced `.expect("semaphore closed")` with `.map_err(|_| DiscoveryError::SemaphoreClosed)?`. Added `SemaphoreClosed` variant to `DiscoveryError`.

#### 2.8 Uplink task — corrupt vs failed message confusion ✅

**Fix**: Separated `valid_ids` from `corrupt_ids`. Corrupt messages are always deleted immediately (preventing head-of-line blocking). Valid messages are only deleted after successful Kafka publish. Failed publishes leave valid messages in the buffer for retry.

---

### 3. Medium-Severity Issues — RESOLVED

#### 3.1 `buffer.rs` — `retry_count` column never used ✅

**Fix**: Removed the unused `retry_count` column from the schema.

#### 3.2 `buffer.rs` — `BufferConfig.max_connections` ignored ✅

**Fix**: Added `BufferStore::with_config(config: BufferConfig)` that respects `max_connections`. The existing `new(path)` method now delegates to `with_config` with defaults.

#### 3.3 `modbus_client.rs` — Exponential backoff overflow ✅

**Fix**: Clamped shift to 63 with `u32::try_from(attempt).unwrap_or(63).min(63)` instead of using `checked_shl` with `unwrap_or(u64::MAX)`.

#### 3.4 `poller_actor.rs` — Jitter correlates across devices ✅

**Fix**: Added a `device_salt` derived from hashing `(ip, unit_id)`. The jitter seed now includes this salt, ensuring different devices get different jitter patterns even when starting simultaneously.

#### 3.5 `poller_actor.rs` — Silent skip of zero-length models ✅

**Fix**: Added `debug!()` log when skipping zero-length models.

#### 3.6 `avro_kafka.rs` — Error context lost in `map_err` ⚠️

**Status**: Not changed. The `apache_avro::Error` type does not implement `Clone` and wrapping it in `PublishError` would require boxing. The `to_string()` approach is acceptable for now; the error message content is preserved.

#### 3.7 `avro_kafka.rs` — Avro schema type mismatch ⚠️

**Status**: Noted but not changed. Avro `"int"` safely widens `u8` and `u16` values. No data loss occurs in practice. A future schema evolution (via Schema Registry) can use more precise types.

#### 3.8 `sunspec_parser.rs` — Limited model name lookup ⚠️

**Status**: Noted for future work. Only 5 models are named. Loading from a resource file is a good enhancement but out of scope for this review.

#### 3.9 `main.rs` — Infinite respawn without backoff ✅

**Fix**: Added per-device `respawn_counts` tracking and a `respawn_delay()` function with exponential backoff (base delay * 2^failures, capped at 60 seconds). Successful exits reset the counter.

#### 3.10 `#![allow(dead_code)]` in every crate ✅

**Fix**: Removed `#![allow(dead_code)]` from all 7 library crates: `types`, `discovery`, `modbus-client`, `sunspec-parser`, `poller-actor`, `avro-kafka`, and `buffer`.

---

### 4. Low-Severity Issues — RESOLVED

#### 4.1 `.gitignore` missing common patterns ✅

**Fix**: Added `.DS_Store`, `.env`, `*.sqlite*`, `.vscode/`, `.idea/`, swap files.

#### 4.2 `rustfmt.toml` is minimal ✅

**Fix**: Added `max_width = 100`, `reorder_imports = true`, `reorder_modules = true`.

#### 4.3 `Cargo.toml` — Tokio `"full"` feature ✅

**Fix**: Replaced `features = ["full"]` with specific features: `["rt-multi-thread", "macros", "io-util", "net", "time", "sync", "signal"]`.

#### 4.4 `deploy-docs.yml` — mdBook version not pinned ✅

**Fix**: Changed from `mdbook-version: 'latest'` to `mdbook-version: '0.4.40'`.

#### 4.5 No `MSRV` specified ✅

**Fix**: Added `rust-version = "1.80"` to `[workspace.package]` in root `Cargo.toml`.

#### 4.6 ARM64 Dockerfile uses `:latest` tag ✅

**Fix**: `docker/Dockerfile.cross` (formerly `Dockerfile.arm64`) pins `ghcr.io/cross-rs/<target>:0.2.5`;
the release workflow builds with it and checks the resulting glibc floor (2.18).

---

### 5. Remaining Future Work

These items were identified but are enhancements, not bugs:

| Item | Priority | Notes |
|------|----------|-------|
| Kafka TLS/SASL config exposure | Medium | rdkafka supports it; need config fields |
| Schema Registry integration | Medium | Hardcoded Avro schema should evolve |
| SunSpec model name resource file | Low | Only 5 models named currently |
| `docker-compose.yml` for dev | Low | Kafka + collector local setup |
| `cargo audit` / `cargo deny` CI | Medium | Security scanning workflow |
| Dependabot configuration | Low | Automated dependency updates |
| `CONTRIBUTING.md`, `CHANGELOG.md` | Low | Community documentation |
| Avro error wrapping (box error) | Low | Preserves full error chain |
| ARM64 Dockerfile version pin | Low | Reproducibility |

---

### 6. Security Observations

| Area | Status | Notes |
|------|--------|-------|
| SQL injection | Safe | Parameterized queries throughout |
| Docker | **Fixed** | Non-root user, healthcheck added |
| Modbus TCP | No TLS | Standard for Modbus protocol |
| Kafka | No TLS/SASL | Config supports it via rdkafka but not exposed |
| Config credentials | Risk | Environment variables could leak in logs |
| systemd service | **Fixed** | Added `NoNewPrivileges`, `ProtectSystem`, `ProtectHome`, `PrivateTmp` |

---

### 7. Performance Observations

| Component | Observation | Impact |
|-----------|------------|--------|
| `ModbusClient` | Single `Mutex<Context>` serializes all reads | Bottleneck under high model count |
| `BufferStore` | Now configurable via `BufferConfig` | **Fixed** — was hardcoded to 5 |
| `uplink_task` | JSON deserialize + Avro re-serialize every cycle | CPU overhead on each batch |
| Discovery | Fresh TCP connection per device | Network overhead at startup |
| Tokio | Specific features only | **Fixed** — was `"full"` |

---

### 8. Test Coverage Assessment

| Crate | Unit Tests | Integration Tests | Gaps |
|-------|-----------|-------------------|------|
| `types` | None | None | Serialization round-trip |
| `sunspec-parser` | Yes | Fixture-based | Edge cases |
| `modbus-client` | None | diagslave | Retry/backoff logic |
| `discovery` | None | None | CIDR parsing edges |
| `poller-actor` | None | None | Error recovery, jitter |
| `avro-kafka` | Serialization | Kafka integration | Mock publisher |
| `buffer` | SQLite ops | None | Concurrent access |
| `collector-app` | Config validation | E2E harness | uplink/buffer tasks |

---

### Files Modified

| File | Changes |
|------|---------|
| `crates/collector-app/src/config.rs` | Added `discovery_unit_ids` field, `unit_ids` to FileDiscoveryConfig, `parse_unit_id_list()` fn, removed duplicate Default fields |
| `crates/collector-app/src/main.rs` | Removed duplicate Prometheus install, separated corrupt/valid IDs in uplink, added respawn backoff |
| `crates/discovery/src/lib.rs` | Added `SemaphoreClosed` error, replaced `expect()`, removed `dead_code` |
| `crates/modbus-client/src/lib.rs` | Clamped backoff shift to 63, removed `dead_code` |
| `crates/poller-actor/src/lib.rs` | Device-salted jitter, debug log for zero-length models, removed `dead_code` |
| `crates/buffer/src/lib.rs` | Added `with_config()`, removed unused `retry_count` column, removed `dead_code` |
| `crates/avro-kafka/src/lib.rs` | Removed `dead_code` |
| `crates/sunspec-parser/src/lib.rs` | Removed `dead_code` |
| `crates/types/src/lib.rs` | Removed `dead_code` |
| `Dockerfile` | Bookworm base, non-root user, healthcheck, `--no-install-recommends` |
| `.gitignore` | Removed `Cargo.lock`, added `.DS_Store`, `.env`, `*.sqlite*`, IDE patterns |
| `.dockerignore` | Removed `Cargo.lock`, added `docs/`, `.github/`, `.env` |
| `Cargo.toml` | Specific tokio features, `rust-version = "1.80"` |
| `rustfmt.toml` | Added `max_width`, `reorder_imports`, `reorder_modules` |
| `.github/workflows/deploy-docs.yml` | Pinned mdBook to `0.4.40` |
| `.github/workflows/ci.yml` | **New** — fmt, clippy, build, test |
