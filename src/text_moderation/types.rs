use serde::{Deserialize, Serialize};
use struct_builder::Builder;

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct ServiceParameters {
    /// 审核的文本内容，限定在600字符以内。
    #[serde(rename = "content")]
    content: Option<String>,
    /// 账户ID，标识一个账户的唯一ID。
    #[serde(rename = "accountId")]
    account_id: Option<String>,
    /// 设备ID，标识一个设备的唯一ID。
    #[serde(rename = "deviceId")]
    device_id: Option<String>,
    /// 通过风险控制SDK获取到的设备令牌。
    #[serde(rename = "deviceToken")]
    device_token: Option<String>,
}

#[derive(Debug, Clone, Builder)]
pub struct TextModerationRequest {
    pub service: Option<String>,
    pub content: Option<String>,
    pub account_id: Option<String>,
    pub device_id: Option<String>,
    pub device_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextModerationResponse {
    #[serde(rename = "Code")]
    pub code: Option<i32>,
    #[serde(rename = "Data")]
    pub data: Option<TextModerationResponseData>,
    #[serde(rename = "Message")]
    pub message: Option<String>,
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextModerationResponseData {
    #[serde(rename = "labels")]
    pub labels: Option<String>,
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(rename = "deviceId")]
    pub device_id: Option<String>,
}