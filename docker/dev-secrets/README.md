# Development secrets

Throwaway credentials for the local `docker compose` stack only. The Kafka broker in
`docker-compose.yml` accepts the `collector` user with this password on its SASL listener,
and the collector reads it from a Docker secret (`SUNSPEC_KAFKA_SASL_PASSWORD_FILE`).

Never use these values outside the local stack.
