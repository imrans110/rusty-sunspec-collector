# Serial devices through a converter

Many inverters and meters only have an RS-485 port. A serial-to-Ethernet converter in
transparent mode (often called "TCP server", "raw" or "transparent"; not "Modbus TCP gateway")
passes Modbus RTU frames between a TCP connection and the bus unchanged. Set the framing to `rtu`
and list each unit on the bus:

```toml
[modbus]
port = 8899                       # the converter's TCP port

[[discovery.static_devices]]
ip = "192.168.1.60"               # the converter
unit_id = 1
framing = "rtu"

[[discovery.static_devices]]
ip = "192.168.1.60"
unit_id = 2
framing = "rtu"
```

(`[modbus] framing = "rtu"` applies it to every device instead, and `SUNSPEC_MODBUS_FRAMING=rtu`
does the same from the environment.)

- RTU has no transaction IDs, so only one request may be on the bus at a time. All units on the
  same converter (host and port) share one TCP connection, and their requests run one after
  another. Per-device poll intervals and model settings may differ, but `request_timeout_ms` and
  `max_batch_size` must match; validation says so if they do not.
- Serial buses are slow: 125 registers at 9600 baud take roughly 300 ms. Raise
  `poller.request_timeout_ms` and, if the converter buffers little, lower `modbus.max_batch_size`.
  A slow or absent unit delays the others, because they wait for the bus.
- A unit that does not answer times out. The connection is then reopened (a late reply could
  otherwise be read as the answer to the next request), so an absent unit on a shared converter
  causes reconnects for its neighbours; remove units that are not on the bus.
- If the converter is in "Modbus TCP gateway" mode instead, it translates Modbus TCP to RTU
  itself: keep the default `framing = "tcp"`.
- `collector-app --probe` uses the same framing, so it works through the converter too.

Native serial ports (a USB RS-485 adapter on the gateway) are not supported yet.
