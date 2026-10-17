## Summary

<!-- What does this change and why? Link the issue it closes, if any (Closes #123). -->

## Checklist

- [ ] `cargo fmt --all -- --check`, `cargo clippy --all-targets --workspace -- -D warnings` and
      `cargo test --workspace` pass
- [ ] Tests cover the change (the `sunspec-sim` simulator can reproduce device behaviour and faults)
- [ ] `CHANGELOG.md` (`[Unreleased]`) and the docs in `docs/` are updated for user-visible changes
- [ ] Breaking changes (record format, metric names, config keys or validation) are marked
      **Breaking:** in the CHANGELOG
