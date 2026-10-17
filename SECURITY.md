# Security Policy

## Supported versions

Security fixes go into the latest release (currently 0.4.x) and `main`. Older versions are not
patched.

## Reporting a vulnerability

Report vulnerabilities privately, not in a public issue:

1. Use **Report a vulnerability** on the repository's
   [Security tab](https://github.com/imrans110/rusty-sunspec-collector/security) (GitHub private
   vulnerability reporting), or
2. contact the maintainer, [@imrans110](https://github.com/imrans110), using the contact details
   on that GitHub profile.

Include the affected version (`collector-app --version`), how it is deployed (release archive,
Docker image or source build), and steps to reproduce. This is a personal project maintained on a
best-effort basis: expect an acknowledgement within a week, and a fix or mitigation plan once the
report is confirmed.

## Scope

In scope: the collector and simulator binaries, the Docker images, `install.sh` and the release
artifacts. Examples: credentials (Kafka SASL passwords, TLS key passphrases) leaking into logs or
error messages, unsafe file permissions set by `install.sh`, or a malicious Modbus device crashing
or hanging the collector.

The collector only reads from devices (Modbus function 0x03); it never writes to them. Modbus TCP
itself has no authentication or encryption, so keep devices on an isolated network; that is not a
vulnerability in this project.
