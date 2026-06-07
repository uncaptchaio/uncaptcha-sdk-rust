use uncaptcha::{UncaptchaAPI, task::WAFTask};

#[tokio::test]
async fn test_waf_task() -> Result<(), Box<dyn std::error::Error>> {
    let api = UncaptchaAPI::new(api_key());
    let payload = WAFTask::builder()
        .url("https://2captcha.com/demo/cloudflare-turnstile-challenge".to_string())
        .proxy(proxy())
        .build();

    let solution = api.execute_task(payload).await?;
    dbg!(&solution);
    Ok(())
}

fn api_key() -> String {
    std::env::var("UNCAPTCHA_API_KEY").expect("UNCAPTCHA_API_KEY env var expected")
}

fn proxy() -> String {
    std::env::var("WAF_PROXY").expect("WAF_PROXY env var expected")
}
