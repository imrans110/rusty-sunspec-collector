# Monitoring and alerts

How to scrape the collector with Prometheus and which alerts to start with. Every metric is
described in [Metrics](../reference/metrics.md).

---

## Scrape configuration

```yaml
scrape_configs:
  - job_name: sunspec-collector
    static_configs:
      - targets: ["collector-host:9090"]
```

The Compose stack uses the same configuration in `docker/prometheus.yml`.

---

## Recommended alert rules

```yaml
groups:
  - name: sunspec-collector
    rules:
      - alert: SunspecCollectorDown
        expr: up{job="sunspec-collector"} == 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Prometheus cannot scrape the collector on {{ $labels.instance }}"

      - alert: SunspecDeviceDown
        expr: sunspec_device_up == 0
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Device {{ $labels.device }} has not been polled successfully for 5 minutes"

      # Devices whose sessions keep failing, including ones that flap between up and down.
      - alert: SunspecDeviceRestarting
        expr: increase(sunspec_poller_restarts_total[15m]) >= 5
        labels:
          severity: warning
        annotations:
          summary: "Device {{ $labels.device }} sessions keep failing"

      - alert: SunspecPollerStalled
        expr: increase(sunspec_poller_stalls_total[15m]) > 0
        labels:
          severity: warning
        annotations:
          summary: "Device {{ $labels.device }} stalled and was restarted"

      - alert: SunspecSamplesDropped
        expr: increase(sunspec_samples_dropped_total[15m]) > 0
        labels:
          severity: warning
        annotations:
          summary: "Samples dropped before the buffer ({{ $labels.reason }})"

      - alert: SunspecUplinkErrors
        expr: increase(sunspec_uplink_errors_total[10m]) > 0
        labels:
          severity: warning
        annotations:
          summary: "Kafka is not acknowledging records from {{ $labels.instance }}"

      # Rows are waiting but nothing was delivered in the last 5 minutes (also fires when
      # nothing has been delivered since startup, when the counter does not exist yet).
      - alert: SunspecUplinkStalled
        expr: (sunspec_buffer_rows > 0) unless (rate(sunspec_uplink_records_total[5m]) > 0)
        for: 10m
        labels:
          severity: critical
        annotations:
          summary: "No records delivered to Kafka from {{ $labels.instance }}"

      - alert: SunspecBufferBacklog
        expr: sunspec_buffer_oldest_age_seconds > 600
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Oldest buffered row is {{ $value | humanizeDuration }} old"

      - alert: SunspecBufferDataLoss
        expr: increase(sunspec_buffer_dropped_total[1h]) > 0
        labels:
          severity: critical
        annotations:
          summary: "Buffered data dropped without being published ({{ $labels.reason }})"
```

To alert on `/healthz` itself, probe it with the Prometheus blackbox exporter (an `http_2xx`
module) and alert on the probe:

```yaml
      - alert: SunspecCollectorUnhealthy
        expr: probe_success{job="sunspec-healthz"} == 0
        for: 5m
        labels:
          severity: critical
```
