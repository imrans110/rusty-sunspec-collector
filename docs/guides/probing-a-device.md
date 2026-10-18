# Probing a device

`collector-app --probe HOST[:UNIT]` reads one device's SunSpec map once and prints it as JSON:
the base address, every model's header address and length, its registers, and the decoded
points. It uses the same settings as the service (`--config`, `SUNSPEC_*` variables, and the
device's static-device overrides), so it shows what the collector would read, and it does not
start the service.

```sh
collector-app --probe 192.168.1.50:126 > dump.json
SUNSPEC_PORT=1502 collector-app --probe inverter.local          # unit defaults to 1
```

The JSON goes to standard output and a one-line summary to standard error, for example
`192.168.1.50/126: SunSpec map at 40000, models 1, 103, 120, 160`. A model that cannot be read is
kept in the dump with its `error`, and named in a warning. Serial numbers (`SN`) and network
addresses (MAC, IPv4, IPv6) in each model's top level are blanked and listed under `redacted`;
`--keep-identifiers` keeps them.

Use it when commissioning a device, when a device is not polled (it shows whether the SunSpec
map is found and every model reads), and for [device reports](https://github.com/imrans110/rusty-sunspec-collector/issues/new?template=device_report.yml).
`sunspec-sim --fixture dump.json` serves the dump again, so a reported problem can be reproduced
without the hardware (see `fixtures/devices/README.md`).
