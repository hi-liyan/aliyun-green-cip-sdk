use crate::green::{GreenClient, NetworkType, Region};
use crate::text_moderation::enums::TextModerationService;
use crate::text_moderation::types::TextModerationRequest;

#[tokio::test]
async fn test_text_moderation() {
    let client = GreenClient::builder()
        .access_key_id("".to_string())
        .access_key_secret("".to_string())
        .region(Region::Shanghai)
        .network_type(NetworkType::External)
        .build();

    let req = TextModerationRequest::builder()
        .service(Some(TextModerationService::NicknameDetection.to_string()))
        .content(Some("测试文本".to_string()))
        .account_id(Some("1001".to_string()))
        .device_id(Some("ios_0".to_string()))
        .build();

    let response = client.text_moderation(req).await;
    println!("response: {:?}", response);
}