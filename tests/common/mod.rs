use restful_booker_automation::api::client::BookerClient;
use restful_booker_automation::utils::config::Config;

pub async fn setup_client() -> BookerClient {
    let config = Config::from_env().expect("config");
    BookerClient::new(config)
}

pub async fn setup_authenticated_client() -> BookerClient {
    let mut client = setup_client().await;
    let config = Config::from_env().expect("config");
    client
        .authenticate(&config.username, &config.password)
        .await
        .expect("auth");
    client
}
