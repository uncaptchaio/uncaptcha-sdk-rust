use uncaptcha::UncaptchaAPI;

#[tokio::test]
async fn test_get_balance() -> Result<(), Box<dyn std::error::Error>> {
    let api = UncaptchaAPI::new(api_key());
    let balance = api.get_balance().await?;
    dbg!(&balance);
    Ok(())
}

fn api_key() -> String {
    std::env::var("UNCAPTCHA_API_KEY").expect("UNCAPTCHA_API_KEY env var expected")
}
