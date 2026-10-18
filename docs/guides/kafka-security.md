# Securing the Kafka connection

The collector supports TLS, mutual TLS and SASL (`PLAIN`, `SCRAM-SHA-256`, `SCRAM-SHA-512`); the
keys are listed in [Configuration](../configuration.md#connection-security-tls-and-sasl). Use
`sasl_ssl` or `ssl` whenever the traffic leaves the site: `sasl_plaintext` sends the password and
the telemetry unencrypted, and the collector logs a warning when it is used.

SCRAM over TLS, with the password in a file:

```toml
[kafka]
brokers = "kafka-1.example.com:9093,kafka-2.example.com:9093"
security_protocol = "sasl_ssl"
ssl_ca_location = "/etc/sunspec-collector/kafka-ca.pem"
sasl_mechanism = "SCRAM-SHA-512"
sasl_username = "site-0042"
sasl_password_file = "/run/credentials/sunspec-collector.service/kafka-password"
```

Mutual TLS instead of SASL: set `security_protocol = "ssl"`, `ssl_ca_location`,
`ssl_certificate_location` and `ssl_key_location` (plus `ssl_key_password_file` if the key is
encrypted).

## Keeping secrets out of config and environment files

Prefer the `*_password_file` settings over putting passwords in the config file or environment:

- **systemd:** store the password root-only on the host and pass it as a credential. Add to the
  unit (or a drop-in from `systemctl edit sunspec-collector`):

  ```ini
  [Service]
  LoadCredential=kafka-password:/etc/sunspec-collector/kafka-password
  Environment=SUNSPEC_KAFKA_SASL_PASSWORD_FILE=/run/credentials/sunspec-collector.service/kafka-password
  ```

  The file is readable only by the service, which also works with `DynamicUser=yes`.
- **Docker / Compose:** mount the password as a secret and point the `_FILE` variable at it, as
  the local stack does:

  ```yaml
  services:
    collector:
      environment:
        SUNSPEC_KAFKA_SASL_PASSWORD_FILE: /run/secrets/kafka_sasl_password
      secrets:
        - kafka_sasl_password
  secrets:
    kafka_sasl_password:
      file: ./kafka-sasl-password
  ```

  The secret file must be readable by the container user (`collector`).

The compose stack's password (`docker/dev-secrets/`) is a development-only value.

## Rotating credentials

Certificates, keys and passwords are read when the collector starts. After rotating them, restart
the service (`systemctl restart sunspec-collector` or `docker compose restart collector`). Unsent
samples stay in the buffer and are published after the restart.
