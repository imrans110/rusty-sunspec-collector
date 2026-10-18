# Verifying a release

From 0.5.0, each release's archives, `install.sh` and `SHA256SUMS`, and both images, carry
[SLSA build provenance](https://slsa.dev/spec/v1.0/provenance): a statement of the workflow run,
commit and tag that built them, signed with a short-lived [Sigstore](https://www.sigstore.dev/)
certificate issued to `release.yml` and recorded in Sigstore's public transparency log. GitHub
artifact attestations are free for public repositories. To check a download with the
[GitHub CLI](https://cli.github.com/):

```sh
gh attestation verify sunspec-collector-0.5.0-x86_64-unknown-linux-gnu.tar.gz \
  --repo imrans110/rusty-sunspec-collector \
  --signer-workflow imrans110/rusty-sunspec-collector/.github/workflows/release.yml \
  --source-ref refs/tags/v0.5.0
gh attestation verify oci://ghcr.io/imrans110/sunspec-collector:0.5.0 \
  --repo imrans110/rusty-sunspec-collector
```

`gh` finds the attestation by the file's or image's digest. The release also has the signed
bundle, `sunspec-collector-<version>.intoto.jsonl`; `--bundle <file>` verifies against it instead
of looking the attestation up on GitHub. Verifying `SHA256SUMS` once covers every archive it lists,
so `install.sh`'s checksum check then ties the installed archive to the attested build.
