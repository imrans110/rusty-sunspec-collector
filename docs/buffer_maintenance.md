# Buffer maintenance

The SQLite buffer is append-only and deletes rows after successful uplink. Over time, the file can grow due to SQLite freelist pages. Schedule a periodic maintenance task if the buffer size grows beyond expectations.

## Recommended strategy

- Keep WAL mode enabled (already configured).
- Run a `VACUUM` during low-traffic windows to reclaim disk space.
- Optionally run `PRAGMA wal_checkpoint(TRUNCATE);` to truncate the WAL file.

## Example maintenance script

```sh
#!/usr/bin/env sh
set -euo pipefail

DB_PATH=${SUNSPEC_BUFFER_PATH:-/var/lib/sunspec-collector/buffer.sqlite}

sqlite3 "$DB_PATH" "PRAGMA wal_checkpoint(TRUNCATE);"
sqlite3 "$DB_PATH" "VACUUM;"
```

## systemd timer (optional)

`/etc/systemd/system/sunspec-buffer-maintenance.service`:

```ini
[Unit]
Description=SunSpec buffer maintenance

[Service]
Type=oneshot
Environment=SUNSPEC_BUFFER_PATH=/var/lib/sunspec-collector/buffer.sqlite
ExecStart=/bin/sh -c 'sqlite3 "$SUNSPEC_BUFFER_PATH" "PRAGMA wal_checkpoint(TRUNCATE);" && sqlite3 "$SUNSPEC_BUFFER_PATH" "VACUUM;"'
```

`/etc/systemd/system/sunspec-buffer-maintenance.timer`:

```ini
[Unit]
Description=Run SunSpec buffer maintenance weekly

[Timer]
OnCalendar=Sun *-*-* 03:00:00
Persistent=true

[Install]
WantedBy=timers.target
```

Enable the timer:

```sh
sudo systemctl daemon-reload
sudo systemctl enable --now sunspec-buffer-maintenance.timer
```
