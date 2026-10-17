# Device fixtures

Each `*.json` file here is a `collector-app --probe` dump: one device's SunSpec register map,
read once. The test `every_device_fixture_replays_to_the_same_dump`
(`crates/collector-app/tests/fixture_replay_tests.rs`) serves every fixture with `sunspec-sim`,
probes it again, and checks that discovery finds the same model chain and that every register
and decoded point matches. A real device's map therefore stays covered by CI.

| File | Device | Source |
|---|---|---|
| `sunspec-sim-demo-inverter.json` | `sunspec-sim` demo inverter (models 1, 103, 120, 160) | Synthetic, captured from the simulator; not a real device |

## Adding a device

1. Probe it. The port, timeout and base address come from your usual config or `SUNSPEC_*`
   variables:

   ```sh
   collector-app --probe 192.168.1.50:126 > vendor-model.json
   ```

2. Check the file before sharing it. Serial numbers (`SN`) and network addresses (MAC, IPv4,
   IPv6) in the top level of each model are blanked and listed under `redacted`. Points inside
   repeating groups, such as battery module serials, are not, so look for anything you would
   rather not publish.
3. Attach it to a [device report](https://github.com/imrans110/rusty-sunspec-collector/issues/new?template=device_report.yml),
   or add it here as `<vendor>-<model>.json` with a row in the table above.

To replay a fixture by hand:

```sh
sunspec-sim --bind 127.0.0.1:1502 --fixture fixtures/devices/vendor-model.json
SUNSPEC_PORT=1502 collector-app --probe 127.0.0.1:<unit>
```

If a decoder change is intentional and alters the decoded points of a fixture, regenerate the
fixture's `points` this way and review the difference.
