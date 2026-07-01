## What does this PR do?

<!-- Describe the change and why it's needed. -->

## Related issue

<!-- Link the issue this addresses, if any. -->

## Checklist

- [ ] Follows the existing client shape (`client.rates().current()`,
      `client.convert().amount(x).from(y).to(z).send()`, etc.) — no parallel
      API surface
- [ ] `cargo build` and `cargo test` pass
- [ ] `cargo clippy -- -D warnings` passes
- [ ] Model changes match the shape served by `tasave-api`
