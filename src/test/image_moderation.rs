use crate::client::{GreenClient, NetworkType, Region};
use crate::image_moderation::enums::ImageModerationService;
use crate::image_moderation::types::ImageModerationRequest;

#[tokio::test]
async fn test_image_moderation() {
    let client = GreenClient::builder()
        .access_key_id("".to_string())
        .access_key_secret("".to_string())
        .region(Region::Shanghai)
        .network_type(NetworkType::External)
        .build();

    let req = ImageModerationRequest::builder()
        .service(Some(ImageModerationService::BaselineCheck.to_string()))
        .image_url(Some("".to_string()))
        .data_id(Some("1001".to_string()))
        .build();

    let response = client.image_moderation(req).await;
    println!("response: {:?}", response);
}