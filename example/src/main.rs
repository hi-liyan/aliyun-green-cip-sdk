use std::env;
use aliyun_green_cip_sdk::client::{GreenClient, NetworkType, Region};
use aliyun_green_cip_sdk::text_moderation::enums::TextModerationService;
use aliyun_green_cip_sdk::text_moderation::types::TextModerationRequest;

#[tokio::main]
async fn main() {
    let client = GreenClient::builder()
        .access_key_id(env::var("ALIYUN_CLOUD_ACCESS_KEY_ID").unwrap())
        .access_key_secret(env::var("ALIYUN_CLOUD_ACCESS_KEY_SECRET").unwrap())
        .region(Region::Shanghai)
        .network_type(NetworkType::External)
        .https(true)
        .build();

    text_scan(&client).await;
}

async fn text_scan(client: &GreenClient) {
    let request_params = TextModerationRequest::builder()
        .service(Some(TextModerationService::NicknameDetection.to_string()))
        .content(Some("我是一个昵称".to_string()))
        .account_id(Some("123456".to_string()))
        .device_id(Some("123456".to_string()))
        .device_token(Some("123456".to_string()))
        .build();

    let response = client.text_moderation(request_params).await;
    println!("response: {:?}", response);
}
