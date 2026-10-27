mod common;

use restful_booker_automation::utils::config::Config;

#[tokio::test]
async fn create_token_success() {
    let config = Config::from_env().unwrap();
    let mut client = common::setup_client().await;
    let auth = client
        .authenticate(&config.username, &config.password)
        .await
        .expect("auth should succeed");
    assert!(!auth.token.is_empty());
}

#[tokio::test]
async fn create_token_invalid_credentials() {
    let mut client = common::setup_client().await;
    let result = client.authenticate("baduser", "badpass").await;
    assert!(result.is_err());
}
