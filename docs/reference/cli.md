# Command-line options

| Option | Effect |
|--------|--------|
| `--config PATH`, `--config=PATH` | Read this config file. Overrides `SUNSPEC_CONFIG`. |
| `--print-schema` | Print the Avro schema of the published records as JSON and exit. No configuration is loaded. |
| `--print-models` | Print the metadata of every bundled SunSpec model (points with type, units, label, description, and enum and bitfield symbols) as JSON and exit. |
| `--probe HOST[:UNIT]` | Read one device's SunSpec map once, print it as JSON and exit; see [Probing a device](../guides/troubleshooting.md#probing-a-device). Uses the port, timeouts, base address and static-device overrides from the configuration. |
| `--keep-identifiers` | With `--probe`: keep serial numbers and network addresses instead of blanking them. |
| `--version`, `-V` | Print `collector-app <version>` and exit. |
| `--help`, `-h` | Print a short usage summary and exit. |

Other arguments are ignored.

---

## Running the binary

```sh
collector-app [--config PATH] [--print-schema]
```

| Argument | Effect |
|----------|--------|
| `--config PATH` (or `--config=PATH`) | Load this TOML or JSON file (`.json` is parsed as JSON, anything else as TOML). `SUNSPEC_CONFIG` does the same. |
| `--print-schema` | Print the Avro schema of the published records to stdout and exit, without reading any configuration |

Other arguments are ignored. Without a config file the collector starts from the compiled defaults
and `SUNSPEC_*` environment variables, which override file values when both are used:

```sh
SUNSPEC_STATIC_DEVICES=192.168.1.20:1,192.168.1.21:1 \
SUNSPEC_KAFKA_BROKERS=kafka.example.internal:9092 \
  ./collector-app
```

Without `SUNSPEC_KAFKA_BROKERS` (or `kafka.brokers`) nothing is sent to Kafka: records are only
logged at debug level. Every setting is described in [Configuration](../configuration.md).

- **Logging:** `RUST_LOG` sets the filter (default `info`, for example `info,poller_actor=debug`);
  `SUNSPEC_LOG_FORMAT=json` writes one JSON object per line (`text` is the default; any other value
  fails at startup). Colours are used only when stdout is a terminal.
- **Files:** the buffer is `sunspec-buffer.sqlite` in the working directory by default, with
  SQLite's `-wal` and `-shm` files next to it. In the Docker image the working directory is
  `/app/data`, so the default is `/app/data/sunspec-buffer.sqlite`; the example systemd
  environment file sets `/var/lib/sunspec-collector/buffer.sqlite`.
- **Ports:** `/metrics` and `/healthz` on `metrics.port` (default 9090). If the port cannot be
  bound, the collector exits at startup with an error containing `bind metrics port <port>`.
- **Signals:** SIGINT and SIGTERM start a graceful shutdown.
- **Exit status:** 0 after a graceful shutdown. Startup errors are printed to stderr, and the
  status says whether trying again can help:
  - **78** (`EX_CONFIG`): a setting is wrong: configuration that does not load or validate, an
    invalid `SUNSPEC_LOG_FORMAT`, or Kafka settings the producer rejects (such as an unreadable CA
    file). The systemd unit does not restart on this status (`RestartPreventExitStatus=78`).
  - **1**: anything else, such as a metrics port that cannot be bound or a buffer that cannot be
    opened. These can pass, so the unit restarts the service.
