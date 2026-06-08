# uncaptcha

Official Rust wrapper for the [uncaptcha.io](https://uncaptcha.io) API.  

## Quick start

```rust
use uncaptcha::{UncaptchaAPI, task::TurnstileTask};

#[tokio::main]
async fn main() -> uncaptcha::Result<()> {
    let api = UncaptchaAPI::new("your-api-key");

    let solution = api
        .execute_task(
            TurnstileTask::builder()
                .url("https://example.com/login")
                .sitekey("0x4AAAAAAA...")
                .build(),
        )
        .await?;

    println!("token: {}", solution.token);
    Ok(())
}
```

## Examples

### Check your balance

```rust
let balance = api.get_balance().await?;
println!("balance: ${balance}");
```

### Turnstile with a proxy and action

All fields besides `url` and `sitekey` are optional. A proxy is preferred for
stability.

```rust
use uncaptcha::task::TurnstileTask;

let solution = api
    .execute_task(
        TurnstileTask::builder()
            .url("https://example.com/")
            .sitekey("0x4AAAAAAA...")
            .action("login")
            .proxy("http://user:pass@1.2.3.4:8080")
            .build(),
    )
    .await?;

println!("token: {}", solution.token);
```

### WAF challenge

`url` and `proxy` are required — the proxy IP must match the one that received
the challenge. If `html` is omitted the API fetches it for you, but providing it
yourself is more reliable.

```rust
use uncaptcha::task::WAFTask;

let solution = api
    .execute_task(
        WAFTask::builder()
            .url("https://protected.example.com/")
            .proxy("socks5://user:pass@1.2.3.4:1080")
            .user_agent("Mozilla/5.0 ...")
            .build(),
    )
    .await?;

println!("clearance: {}", solution.clearance);
println!("cf_rt: {}", solution.cf_rt);
```

## Errors

Every call returns `uncaptcha::Result<T>`. Errors are either transport failures
(`Error::Request`) or API-side failures (`Error::API`).

```rust
match api.execute_task(task).await {
    Ok(sol) => println!("solved: {}", sol.token),
    Err(e) => eprintln!("failed: {e}"),
}
```
