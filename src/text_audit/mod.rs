use serde::{Deserialize, Serialize};
use struct_builder::Builder;

/// # 审核服务类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Service {
    NicknameDetection,
    ChaDetection,
    CommentDetection,
    AiArtDetection,
    AdComplianceDetection,
    PgcDetection,
}

impl Service {
    pub fn as_str(self) -> &'static str {
        match self {
            Service::NicknameDetection => "nickname_detection",
            Service::ChaDetection => "chat_detection",
            Service::CommentDetection => "comment_detection",
            Service::AiArtDetection => "ai_art_detection",
            Service::AdComplianceDetection => "ad_compliance_detection",
            Service::PgcDetection => "pgc_detection"
        }
    }
}

/// # 请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct RequestParams {
    #[serde(rename = "Service")]
    service: Option<Service>,
    #[serde(rename = "ServiceParameters")]
    service_parameters: Option<ServiceParameters>
}

/// # 审核服务需要的参数集
///
/// JSON字符串格式，关于每个字符串的描述。
///
/// 参考文档：
/// - https://help.aliyun.com/document_detail/434034.html?spm=a2c4g.434034.0.0.115c41ed368uSK#table-3ji-jc0-y0o
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
    device_token: Option<String>
}

/// # 返回参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "Code")]
    pub code: Option<i32>,
    #[serde(rename = "Data")]
    pub data: Option<ResponseData>,
    #[serde(rename = "Message")]
    pub message: Option<String>,
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>
}

/// # 返回参数中的 Data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseData {
    #[serde(rename = "Labels")]
    pub labels: Option<String>,
    #[serde(rename = "Reason")]
    pub reason: Option<String>,
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    #[serde(rename = "DeviceId")]
    pub device_token: Option<String>
}

