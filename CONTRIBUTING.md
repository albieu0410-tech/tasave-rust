# Contributing to tasave-rust

Thanks for considering a contribution — this SDK is MIT licensed and
community input is welcome.

## Before you start
- For anything non-trivial, open an issue first to discuss the approach
  before writing code.
- The response shapes here must match the `Rate`/`BcvRate`/`ParallelRate`/
  `ConvertResult`/`HistoryEntry`/`Status` models served by the
  [tasave-api](https://github.com/albieu0410-tech/tasaVE) server — check
  that repo's `CLAUDE.md` before changing a model.

## Development setup
```bash
cargo build
cargo test
cargo clippy -- -D warnings
```

## Pull requests
- Keep PRs focused — one logical change per PR.
- Match the existing client shape: `client.rates().current()`,
  `client.convert().amount(x).from(y).to(z).send()`, etc. Don't introduce a
  parallel API surface.
- Run `cargo clippy` before finishing — fix all warnings, no exceptions.
- Describe *why*, not just *what*, in the PR description.

## Code of conduct
Be respectful. Keep contributions focused on making the SDK a reliable,
faithful client for the TasaVE API.
