# tasave

Rust client for [TasaVE](https://tasave.sudelca.com) — Venezuelan dollar
exchange rate data (BCV official rate + parallel/P2P rates), multi-source
with confidence scoring.

## Install

```bash
cargo add tasave-rust
```

## Usage

```rust
let client = tasave::TasaVE::new(); // no API key needed for public endpoints
let rate = client.rates().current().await?;

println!("{}", rate.bcv_usd);     // 104.23
println!("{}", rate.confidence);  // 92.0
```

With an API key (needed for `.rates().parallel()` and `.history()`):

```rust
let client = tasave::TasaVE::with_key("tv_live_...");
let parallel = client.rates().parallel().await?;
```

## Other endpoints

```rust
client.rates().bcv().await?;                 // BCV official only

client.convert()
    .amount(100.0)
    .from("USD")
    .to("VES")
    .send()
    .await?;

client.history().range("2026-06-01", "2026-06-30").await?;
client.history().date("2026-06-30").await?;
client.status().await?;
```

## Errors

```rust
match client.rates().parallel().await {
    Ok(rate) => { /* ... */ }
    Err(tasave::Error::Api { status, message }) => { /* e.g. 401, "API key required" */ }
    Err(tasave::Error::Http(e)) => { /* network/transport error */ }
    Err(tasave::Error::MissingParam(name)) => { /* convert() builder missing a required field */ }
}
```

## License

MIT
