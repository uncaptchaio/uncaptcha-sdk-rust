use uncaptcha::{UncaptchaAPI, task::TurnstileTask};

#[tokio::test]
async fn test_turnstile_task() -> Result<(), Box<dyn std::error::Error>> {
    let api = UncaptchaAPI::new(api_key());
    let payload = TurnstileTask::builder()
        .sitekey("0x4AAAAAAAA1WzHSsbGWTH88".to_string())
        .url("https://turnstile-test.vercel.app/".to_string())
        .build();

    let solution = api.execute_task(payload).await?;
    dbg!(&solution);
    Ok(())
}

fn api_key() -> String {
    std::env::var("UNCAPTCHA_API_KEY").expect("UNCAPTCHA_API_KEY env var expected")
}
