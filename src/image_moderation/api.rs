use std::time::Duration;
use reqwest::Method;
use uuid::Uuid;
use crate::client::GreenClient;
use crate::error::{ImageModerationError, SdkError};
use crate::image_moderation::types::{ImageModerationRequest, ImageModerationResponse, ServiceParameters};
use crate::utils::get_utc;

impl GreenClient {
    /// 图片审核增强版API
    ///
    /// 参考文档：
    /// - https://help.aliyun.com/document_detail/467829.html?spm=a2c4g.434034.0.0.23813d12NZGIrI
    pub async fn image_moderation(&self, req: ImageModerationRequest) -> Result<ImageModerationResponse, SdkError> {
        let date = get_utc();
        let signature_nonce = Uuid::new_v4().to_string();

        if req.service.is_none() {
            return Err(SdkError::ImageModerationError(ImageModerationError::ServiceNotSet))
        }
        if !(req.image_url.is_some() || (req.oss_bucket_name.is_some() && req.oss_object_name.is_some())) {
            return Err(SdkError::ImageModerationError(ImageModerationError::ImageNotSet))
        }

        let service_params = ServiceParameters::builder()
            .image_url(req.image_url)
            .oss_bucket_name(req.oss_bucket_name)
            .oss_object_name(req.oss_object_name)
            .oss_region_id(req.oss_region_id)
            .data_id(req.data_id)
            .referer(req.referer)
            .build();

        let service_params = serde_json::to_string(&service_params).unwrap();

        // 请求参数
        let mut query_parameters = vec![
            ("Format", "JSON"),
            ("Version", "2022-03-02"),
            ("SignatureMethod", "Hmac-SHA1"),
            ("SignatureNonce", signature_nonce.as_str()),
            ("SignatureVersion", "1.0"),
            ("Action", "ImageModeration"),
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
            .timeout(Duration::from_millis(self.timeout))
            .build()
            .unwrap();

        let response = self.client.execute(request).await?;
        let response = response.json().await?;
        Ok(response)
    }
}