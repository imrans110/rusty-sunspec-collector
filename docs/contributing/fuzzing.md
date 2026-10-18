# Fuzzing

`fuzz/` holds [cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz.html) targets for the
code that reads input the collector does not control:

| Target | Input | Checks |
|--------|-------|--------|
| `decode_model` | A bundled model ID and arbitrary registers | `decode_model_labeled` does not panic, agrees with `decode_model`, names each point once, labels only existing points, and returns only finite floats |
| `model_chain` | An arbitrary register map at the configured or a standard base | `discover_models` ends with a chain or an error; an accepted chain starts with `SunS`, has at most 256 contiguous models and ends with `0xFFFF`; each model it finds decodes without panicking |
| `config` | An arbitrary config file, loaded as TOML and as JSON | `CollectorConfig::from_file_contents`, `validate()` and `device_config()` return errors instead of panicking |

`discover_models` reads through the `ReadRegisters` trait, which the target implements with an
in-memory register map, so the chain walk runs without a network. The config target skips inputs
that could name a `*_file` setting, since loading reads those files.

`fuzz/` is its own workspace (it needs nightly Rust and sanitizers), so the workspace commands
above do not build it. It has no committed lockfile: builds copy the workspace's `Cargo.lock`
into `fuzz/` first, so the targets use the same dependency versions as the collector, and only
one lockfile needs updates. To run a target locally:

```sh
rustup toolchain install nightly
cargo install cargo-fuzz --locked
cp Cargo.lock fuzz/                          # build against the workspace's locked versions
fuzz/seeds.py fuzz/seeds                     # seed corpora from fixtures/ and the example config
cargo +nightly fuzz run -O decode_model fuzz/corpus/decode_model fuzz/seeds/decode_model \
  -- -max_total_time=300
```

A crash is saved under `fuzz/artifacts/<target>/`; `cargo +nightly fuzz run -O <target> <file>`
replays it and `cargo +nightly fuzz tmin -O <target> <file>` shrinks it. Turn a fixed crash into
a regression test in the crate's tests (see `values_that_are_not_finite_are_null` in
`crates/sunspec-parser/tests/decode_tests.rs`, found by `decode_model`).

In CI, `.github/workflows/fuzz.yml` uses [ClusterFuzzLite](https://google.github.io/clusterfuzzlite/):
it builds the targets in the OSS-Fuzz Rust image (`.clusterfuzzlite/`) with AddressSanitizer and
seed corpora from `fuzz/seeds.py`. It fuzzes for two minutes on pull requests that change the
fuzzed code (`crates/sunspec-parser/`, `crates/poller-actor/`, `crates/collector-app/src/config.rs`)
or the fuzzing setup (`fuzz/`, `.clusterfuzzlite/`, the workflow), and for half an hour every
Sunday, which also covers dependency updates; the corpus is kept between runs as workflow
artifacts. A crash fails the run and is uploaded with its input. When you add a target, add a
`[[bin]]` to `fuzz/Cargo.toml`, add it to `targets` in `.clusterfuzzlite/build.sh`, add the code
it fuzzes to the `paths` in `fuzz.yml`, and give it seeds in `fuzz/seeds.py` if the fixtures have
useful input.
