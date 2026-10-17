# Buffer Maintenance

The collector writes every sample to a local SQLite database before it publishes anything to
Kafka. This page describes what is in that database, how rows move through it, how to size it,
and how to inspect and maintain it safely.

Settings referred to here are described in [Configuration](configuration.md#buffer); metrics and
alerts are in [Operations](ops.md#metrics-reference).

---

## What the buffer is for

Pollers produce samples on a fixed schedule whether or not Kafka is reachable. The buffer
decouples the two:

```
pollers → channel → ingest → SQLite buffer → uplink → Kafka
```

- **Ingest** writes samples to SQLite in transactions of up to 256 rows: samples that arrive
  within 100 ms of the first one share one transaction and one fsync.
- **Uplink** reads the oldest rows, publishes them, and deletes them once Kafka has acknowledged
  them.
- **Retention** runs every 10 s and keeps the buffer within `buffer.max_rows` and
  `buffer.max_age_s`.

During a Kafka or network outage the rows wait on disk and are sent when Kafka is reachable again.
The buffer also survives collector restarts.

---

## Where it lives

| Deployment | Path | Notes |
|------------|------|-------|
| Local run | `sunspec-buffer.sqlite` | Default; relative to the working directory. |
| Docker image | `/app/data/sunspec-buffer.sqlite` | The default relative path, resolved against the image's working directory `/app/data`. Mount a volume at `/app/data`; the Compose stack uses the volume `collector-data` (listed by `docker volume ls` with the project name as prefix). A relative `buffer.path` or `SUNSPEC_BUFFER_PATH` also lands in `/app/data`. |
| systemd | `/var/lib/sunspec-collector/buffer.sqlite` | Set in `/etc/sunspec-collector.env`. With `DynamicUser=yes`, `/var/lib/sunspec-collector` is a symlink to `/var/lib/private/sunspec-collector`, which only root can enter. |

The path is set with `buffer.path` / `SUNSPEC_BUFFER_PATH`. The file is created if it does not
exist, and the table and index are created at startup.

SQLite keeps three files side by side, named after the database file (here the systemd name
`buffer.sqlite`; in Docker by default `sunspec-buffer.sqlite`, `sunspec-buffer.sqlite-wal` and
`sunspec-buffer.sqlite-shm`):

| File | Contents |
|------|----------|
| `buffer.sqlite` | The database. |
| `buffer.sqlite-wal` | Write-ahead log. Holds recently committed transactions that have not yet been copied into the main file. |
| `buffer.sqlite-shm` | Shared-memory index for the WAL. |

Treat the three files as one unit: copy, move or delete them together, and only while the collector
is stopped.

---

## Schema

```sql
CREATE TABLE telemetry_queue (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    topic      TEXT    NOT NULL,
    payload    BLOB    NOT NULL,
    created_at INTEGER NOT NULL
);
CREATE INDEX idx_created_at ON telemetry_queue(created_at);
```

| Column | Contents |
|--------|----------|
| `id` | Increasing row ID. The uplink publishes in `id` order, oldest first. |
| `topic` | Kafka topic configured when the row was written. The uplink publishes every row to the topic configured *now*, so after changing `kafka.topic` the backlog goes to the new topic. |
| `payload` | The sample as UTF-8 JSON (stored as a BLOB). It is converted to Avro only when published. |
| `created_at` | Time the row was written, in Unix milliseconds. Retention and `sunspec_buffer_oldest_age_seconds` use it. All rows from one ingest transaction share the same value. |

A payload looks like this (registers and points shortened):

```text
{
  "device": {"ip": "192.168.1.20", "unit_id": 1},
  "model_id": 103,
  "model_name": "inverter_three_phase",
  "start": 40070,
  "registers": [103, 50, 769, 0, 0, 0, 65534, ...],
  "collected_at_ms": 1792212700844,
  "points": [["A", 7.69], ["AphA", 0.0], ..., ["PhVphA", 230.3], ["W", 5309.0], ["Hz", 50.02], ["St", 4]]
}
```

- `start` is the register address of the model header; `registers` are the raw values, including
  the two header registers.
- `collected_at_ms` is the time the model was read. It differs from `created_at`, which is when
  the row was written.
- `points` are the decoded points as `[name, value]` pairs in definition order. A value is a
  number, a string or `null` (not implemented). Models without a bundled definition have an empty
  list. Rows written by earlier versions have no `points` field and are published with an empty
  `points` map.

### SQLite settings

- **WAL journal mode.** Readers and the writer do not block each other, so the uplink can read
  while ingest writes, and a `sqlite3` session can read while the collector runs. Writes (ingest
  inserts, uplink and retention deletes) still take turns.
- **`synchronous=NORMAL`**, set on every connection. A committed transaction survives a crash or
  kill of the collector process. After a power loss or an operating-system crash, the most recent
  transactions can be rolled back; the database itself stays consistent.
- SQLite checkpoints the WAL into the main file automatically, by default every 1000 pages
  (about 4 MB with the default 4 KB page size), and reuses the WAL file afterwards.

---

## Delivery semantics

Delivery is **at-least-once**.

1. The uplink reads up to `buffer.batch_size` rows, oldest first.
2. Each row becomes one Kafka record, keyed by `device_key` (`host/unit`). All records of the batch
   are handed to the producer, then the uplink waits for the acknowledgements.
3. Rows that Kafka acknowledged are deleted.
4. Rows that Kafka did not acknowledge within `kafka.timeout_ms` stay in the buffer. They are
   still the oldest rows, so they are retried first. Failed records count in
   `sunspec_uplink_errors_total`. After a batch in which nothing was delivered, the uplink backs
   off (1 s, doubling, up to 30 s).
5. Rows that can never be published (the payload is not valid JSON, or the sample cannot be
   encoded as Avro) are deleted, logged, and counted in
   `sunspec_buffer_dropped_total{reason="corrupt"}`, so they cannot block the queue.

While a backlog exists, the uplink drains batch after batch without pausing. It waits
`buffer.drain_interval_ms` only after a batch that was not full.

Duplicates are possible. If the process dies after Kafka acknowledged a batch but before the rows
were deleted, or if the delete fails (logged as `deleting published rows failed; they may be sent
again`), those rows are sent again. Consumers can deduplicate on
**`(device_key, model_id, collected_at_ms)`**.

Records are sent oldest first, but a record that fails and is retried can arrive after newer
records. Consumers should order by `collected_at_ms`, not by arrival.

Without `kafka.brokers`, the uplink hands records to a log-only sink, which counts as delivered:
rows are deleted and nothing is kept.

---

## Retention

The retention task runs at startup and then every 10 s:

1. Deletes rows whose `created_at` is older than `buffer.max_age_s` (default 604800 s, 7 days).
   Counted in `sunspec_buffer_dropped_total{reason="max_age"}`.
2. Deletes the oldest rows beyond `buffer.max_rows` (default 1,000,000). Counted in
   `sunspec_buffer_dropped_total{reason="max_rows"}`.
3. Updates `sunspec_buffer_rows` and `sunspec_buffer_oldest_age_seconds`.

Both deletions log a warning with the number of rows dropped. Rows dropped by retention are lost.

Because retention runs every 10 s, the buffer can briefly exceed `max_rows` by up to 10 s of
writes. Because it runs at startup, rows older than `max_age_s` are deleted when a collector starts
after a long stop, before the uplink can send them.

Samples that never reach the buffer are counted separately:
`sunspec_samples_dropped_total{reason="channel_full"}` (ingest too slow) and
`sunspec_buffer_dropped_total{reason="write_error"}` (an SQLite write failed, for example on a full
disk).

---

## Sizing

### Rows per second

Every polled model of every device produces one row per poll interval, and a model with a
[polling profile](configuration.md#polling-profiles) interval one row per interval:

```
rows per second = devices × (models polled every cycle ÷ poll interval in seconds
                             + Σ 1 ÷ slow model interval in seconds)
```

The `discovered SunSpec models` log line lists each device's models, the ones it polls and the
slow ones. For example, a device with models 1, 103, 120, 121 and 160, polled every second with
1 and 120 hourly and 121 every 10 minutes, produces about 2 rows per second instead of 5.

### Bytes per row

Row size depends on the models. As a rough upper estimate:

```
payload bytes ≈ 150 + 5 × registers + 15 × decoded points
bytes on disk ≈ payload bytes + about 120 (row header, topic, timestamp, index entry)
```

For reference, with the simulator's models 1, 103, 120 and 160 the payloads were 436–718 bytes
(about 580 on average) and the database file grew by about 0.7 KB per row. Rows from real devices
can be larger, because values with more digits take more bytes in JSON. To measure your own rows, see
[Inspecting the buffer](#inspecting-the-buffer).

### Example

10 inverters × 4 models, polled every second, at about 0.7 KB per row:

| | Value |
|-|-------|
| Rows per second | 10 × 4 ÷ 1 = 40 |
| Rows per hour | 144,000 (about 100 MB) |
| Rows per day | 3,456,000 (about 2.4 GB) |
| Outage covered by the default `max_rows` (1,000,000) | 1,000,000 ÷ 40 ≈ 25,000 s ≈ 7 hours, about 0.7 GB |
| `max_rows` for a 24-hour outage | 40 × 86,400 ≈ 3,500,000, about 2.4 GB |

With these numbers the default `max_rows` is reached long before the default `max_age_s` of 7
days. Pick the outage you want to ride out, then:

```
max_rows   ≥ rows per second × outage seconds
disk space ≥ max_rows × bytes on disk + headroom for the WAL and for VACUUM (see below)
```

The database file does not shrink when rows are deleted. SQLite reuses the freed pages, so after
a large backlog has drained the file keeps its largest size until it is vacuumed.

---

## Inspecting the buffer

Use the `sqlite3` command-line tool (`apt install sqlite3` on Debian and Ubuntu). The Docker image
does not include it.

While the collector is running, open the database read-only and keep sessions short. A session
left open holds a read snapshot that stops SQLite from checkpointing the WAL, so the WAL file
grows. `-readonly` fails with `unable to open database file` when the `-wal` and `-shm` files do
not exist; in that case stop the collector and follow [Safe maintenance](#safe-maintenance).

**systemd** (root is needed to enter `/var/lib/private`):

```sh
sudo sqlite3 -readonly /var/lib/sunspec-collector/buffer.sqlite
```

**Docker**, from a throwaway container that mounts the same volume:

```sh
docker run --rm -it -v sunspec-data:/data debian:bookworm-slim sh -c \
  'apt-get update -qq && apt-get install -y -qq sqlite3 >/dev/null && sqlite3 -readonly /data/sunspec-buffer.sqlite'
```

The examples use the Docker default file name. If you set `buffer.path` or `SUNSPEC_BUFFER_PATH`
(for example `/app/data/buffer.sqlite`), use the matching name under `/data`.

The queries below can be typed at the `sqlite3>` prompt, or passed on standard input:

```sh
sudo sqlite3 -readonly /var/lib/sunspec-collector/buffer.sqlite <<'SQL'
SELECT COUNT(*) FROM telemetry_queue;
SQL
```

Rows waiting:

```sql
SELECT COUNT(*) AS rows FROM telemetry_queue;
```

Age of the oldest row (the same value as `sunspec_buffer_oldest_age_seconds`):

```sql
SELECT datetime(MIN(created_at) / 1000, 'unixepoch') AS oldest_written_utc,
       round((CAST(strftime('%s', 'now') AS INTEGER) * 1000 - MIN(created_at)) / 1000.0, 1) AS oldest_age_s
FROM telemetry_queue;
```

Rows per device and model:

```sql
SELECT json_extract(CAST(payload AS TEXT), '$.device.ip') || '/' ||
       json_extract(CAST(payload AS TEXT), '$.device.unit_id') AS device,
       json_extract(CAST(payload AS TEXT), '$.model_id') AS model_id,
       COUNT(*) AS rows
FROM telemetry_queue
GROUP BY device, model_id
ORDER BY device, model_id;
```

The oldest rows, and one payload:

```sql
SELECT id, datetime(created_at / 1000, 'unixepoch') AS written_utc, length(payload) AS bytes
FROM telemetry_queue ORDER BY id LIMIT 5;

SELECT CAST(payload AS TEXT) FROM telemetry_queue ORDER BY id LIMIT 1;
```

Average row size and file usage, for [sizing](#sizing):

```sql
SELECT COUNT(*) AS rows, round(avg(length(payload))) AS avg_payload_bytes FROM telemetry_queue;

SELECT page_count * page_size AS file_bytes, freelist_count * page_size AS free_bytes
FROM pragma_page_count(), pragma_page_size(), pragma_freelist_count();
```

`CAST(payload AS TEXT)` makes the JSON functions treat the BLOB as text JSON on every SQLite
version.

---

## Safe maintenance

**Stop the collector before changing the buffer.** Reading is safe while it runs; anything that
writes (VACUUM, DELETE, replacing the file) should be done with the service stopped.

```sh
sudo systemctl stop sunspec-collector     # systemd
docker stop sunspec-collector             # Docker
```

Commands run as root can leave a root-owned `-wal` or `-shm` file behind, which the service user
then cannot open. Each procedure below ends by giving the files back to the owner of the
directory.

### Reclaim disk space (VACUUM)

Routine vacuuming is not needed: SQLite reuses freed pages. Vacuum when a large backlog has
drained and you want the disk space back. VACUUM rewrites the whole database and needs free space
of about twice the database size.

Do not vacuum while the collector is running. VACUUM holds an exclusive lock for its whole run; the
collector's writes wait for a few seconds and then fail, and the samples in them are dropped
(`reason="write_error"`).

systemd:

```sh
sudo systemctl stop sunspec-collector
sudo sqlite3 /var/lib/sunspec-collector/buffer.sqlite 'VACUUM; PRAGMA wal_checkpoint(TRUNCATE);'
sudo sh -c 'chown --reference=/var/lib/sunspec-collector /var/lib/sunspec-collector/*'
sudo systemctl start sunspec-collector
```

Docker:

```sh
docker stop sunspec-collector
docker run --rm -v sunspec-data:/data debian:bookworm-slim sh -c '
  apt-get update -qq && apt-get install -y -qq sqlite3 >/dev/null &&
  sqlite3 /data/sunspec-buffer.sqlite "VACUUM; PRAGMA wal_checkpoint(TRUNCATE);" &&
  chown --reference=/data /data/*'
docker start sunspec-collector
```

Earlier versions of this guide suggested a weekly systemd timer that vacuumed the live database.
Remove such a timer, or make it stop the service first, for the reason above.

### Discard part of the backlog

Deleted rows are never published. With the collector stopped:

```sql
-- Rows written more than 24 hours ago
DELETE FROM telemetry_queue WHERE created_at < (CAST(strftime('%s', 'now') AS INTEGER) - 86400) * 1000;

-- All rows of one device
DELETE FROM telemetry_queue
WHERE json_extract(CAST(payload AS TEXT), '$.device.ip') = '192.168.1.20'
  AND json_extract(CAST(payload AS TEXT), '$.device.unit_id') = 1;
```

Then restore ownership as shown above and start the collector.

### Delete the buffer

Deleting the buffer loses every sample that has not been published. With the collector stopped,
delete the database file and its `-wal` and `-shm` files together (for example `buffer.sqlite`,
`buffer.sqlite-wal` and `buffer.sqlite-shm`); the collector creates
an empty buffer at the next start. Never delete only the `-wal` file: it can hold committed rows
that are not yet in the main file.

### Back up

A buffer backup is rarely needed, since its rows are meant to be short-lived. If you need one:

- **While running:** use SQLite's online backup, which produces a consistent copy:

  ```sh
  sudo sqlite3 -readonly /var/lib/sunspec-collector/buffer.sqlite ".backup '/root/buffer-backup.sqlite'"
  ```

- **While stopped:** copy all three files.

Restoring a backup (collector stopped, old files removed, ownership restored) re-sends every row
in it, including rows that were delivered after the backup was taken. Consumers see duplicates.

---

## Recovery after a long outage

While Kafka is unreachable:

- the uplink retries the oldest batch with backoff (1 s doubling to 30 s), and each attempt waits
  up to `kafka.timeout_ms` for acknowledgements. `sunspec_uplink_errors_total` increases with each
  attempt;
- polling and ingest carry on, so `sunspec_buffer_rows` and `sunspec_buffer_oldest_age_seconds`
  rise;
- once the limits are reached, retention drops the oldest rows
  (`sunspec_buffer_dropped_total{reason="max_rows"|"max_age"}`);
- `/healthz` stays 200, because the pipeline itself is working.

When Kafka is reachable again:

1. The next attempt (at most 30 s later) succeeds, and the uplink then drains back to back while
   the batches come back full. It pauses for `buffer.drain_interval_ms` only once a batch is not
   full, that is, when the backlog is gone.
2. Rows are sent oldest first, so the newest data arrives last. Consumers receive records with old
   `collected_at_ms` values for a while.
3. Watch `sunspec_buffer_rows` and `sunspec_buffer_oldest_age_seconds` fall. The drain only makes
   progress while delivery is faster than intake:

   ```promql
   rate(sunspec_uplink_records_total[5m]) - rate(sunspec_buffer_writes_total[5m])
   ```

   The remaining time in seconds is roughly:

   ```promql
   sunspec_buffer_rows
     / (rate(sunspec_uplink_records_total[5m]) - rate(sunspec_buffer_writes_total[5m]))
   ```

4. To drain faster, raise `buffer.batch_size` (for example to 500 or 1000) and restart the
   collector; buffered rows are kept across the restart. Each batch waits for the producer's 50 ms
   linger plus the broker round trip, so larger batches mean fewer waits. One batch is held in
   memory at a time.
5. Data that retention dropped during the outage is gone; `sunspec_buffer_dropped_total` shows
   how many rows.
6. After the backlog has drained, the database file keeps its largest size. Vacuum it if you need
   the disk space back.

Do not delete the buffer to catch up faster unless you intend to discard the backlog.
