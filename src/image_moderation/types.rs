use serde::{Deserialize, Serialize};
use struct_builder::Builder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestParams {
    #[serde(rename = "Service")]
    service: Option<String>,
    #[serde(rename = "ServiceParameters")]
    service_parameters: Option<ServiceParameters>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct ServiceParameters {
    /// 待检测对象的URL，请确保该URL能通过公网访问到，且URL地址长度不超过2048个字符。
    ///
    /// 说明：URL地址中不能包含中文，且一次请求请确保仅传入1条URL。
    #[serde(rename = "imageUrl")]
    image_url: Option<String>,
    /// 已授权OSS空间的Bucket名。
    ///
    /// 说明：使用OSS图片内网地址时必须先使用阿里云账号（即主账号）访问云资源访问授权页面进行授权。
    #[serde(rename = "ossBucketName")]
    oss_bucket_name: Option<String>,
    /// 已授权OSS空间的文件名。
    #[serde(rename = "ossObjectName")]
    oss_object_name: Option<String>,
    /// OSS Bucket所在区域。
    #[serde(rename = "ossRegionId")]
    oss_region_id: Option<String>,
    /// 检测对象对应的数据ID。
    ///
    /// 由大小写英文字母、数字、下划线（_）、短划线（-）、英文句号（.）组成，不超过64个字符，可以用于唯一标识您的业务数据。
    ///
    /// 示例值
    /// - img123****
    #[serde(rename = "dataId")]
    data_id: Option<String>,
    /// referer请求头，用于防盗链等场景。长度不超过256个字符。
    ///
    /// 示例值
    /// - www.aliyun.com
    #[serde(rename = "referer")]
    referer: Option<String>
}

#[derive(Debug, Clone, Builder)]
pub struct ImageModerationRequest {
    pub service: Option<String>,
    pub image_url: Option<String>,
    pub oss_bucket_name: Option<String>,
    pub oss_object_name: Option<String>,
    pub oss_region_id: Option<String>,
    pub data_id: Option<String>,
    pub referer: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageModerationResponse {
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
    #[serde(rename = "Data")]
    pub data: Option<ImageModerationData>,
    #[serde(rename = "Code")]
    pub code: Option<i32>,
    #[serde(rename = "Msg")]
    pub message: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageModerationData {
    #[serde(rename = "DataId")]
    pub data_id: Option<String>,
    #[serde(rename = "Result")]
    result: Option<Vec<ImageModerationDataResult>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageModerationDataResult {
    /// 图片内容检测运算后返回的标签。同一张图片可能会检出多个标签和分值。
    ///
    /// 参考文档：
    /// - https://help.aliyun.com/document_detail/467829.html?spm=a2c4g.467828.0.0.507a4298anhBEe
    #[serde(rename = "Label")]
    pub label: Option<String>,
    /// 置信分值，0到100分，保留到小数点后2位。部分标签无置信分，更多信息，请参见[风险标签释义表](https://help.aliyun.com/document_detail/467829.html?spm=a2c4g.467828.0.0.507a4298anhBEe#section-2e7-9gb-j7w)。
    #[serde(rename = "Confidence")]
    pub confidence: Option<f32>
}