# Development secrets

Throwaway credentials for the local `docker compose` stack only:

- `kafka_server_jaas.conf`: the broker's SASL/PLAIN users (mounted into the Kafka container
  and referenced from `KAFKA_OPTS`).
- `kafka-sasl-password`: the `collector` user's password, which the collector reads from a
  Docker secret (`SUNSPEC_KAFKA_SASL_PASSWORD_FILE`).

Never use these values outside the local stack.
