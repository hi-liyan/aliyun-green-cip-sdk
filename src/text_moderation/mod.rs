use reqwest::Method;
use uuid::Uuid;

use crate::error::{SdkError, TextModerationError};
use crate::client::GreenClient;
use crate::text_moderation::types::{ServiceParameters, TextModerationRequest, TextModerationResponse};
use crate::utils::get_utc;

pub mod types;
pub mod enums;

impl GreenClient {
    /// 文本审核
    pub async fn text_moderation(&self, req: TextModerationRequest) -> Result<TextModerationResponse, SdkError> {
        let date = get_utc();
        let signature_nonce = Uuid::new_v4().to_string();

        if req.service.is_none() {
            return Err(SdkError::BadParams(TextModerationError::ServiceNotSet))
        }
        if req.content.is_none() {
            return Err(SdkError::BadParams(TextModerationError::ContentNotSet))
        }

        let service_params = ServiceParameters::builder()
            .content(req.content)
            .account_id(req.account_id)
            .device_id(req.device_id)
            .device_token(req.device_token)
            .build();

        let service_params = serde_json::to_string(&service_params).unwrap();

        // 请求参数
        let mut query_parameters = vec![
            ("Format", "JSON"),
            ("Version", "2022-03-02"),
            ("SignatureMethod", "Hmac-SHA1"),
            ("SignatureNonce", signature_nonce.as_str()),
            ("SignatureVersion", "1.0"),
            ("Action", "TextModeration"),
            ("AccessKeyId", self.access_key_id.as_str()),
            ("Timestamp", date.as_str()),
            ("Service", req.service.as_ref().unwrap().as_str()),
            ("ServiceParameters", service_params.as_str()),
        ];

        // 获取签名
        let signature = self.generate_signature("POST", &query_parameters);
        query_parameters.push(("Signature", signature.as_str()));

        let request = self.client
            .request(Method::POST, self.endpoint.as_str())
            .query(&query_parameters)
            .build()
            .unwrap();

        let response = self.client.execute(request).await?;
        let response = response.json().await?;
        Ok(response)
    }
}