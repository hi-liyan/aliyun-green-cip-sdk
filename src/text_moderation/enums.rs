/// 文本审核服务名称
///
/// 参考文档：
/// - https://help.aliyun.com/document_detail/464388.html?spm=a2c4g.434034.0.0.4bfc3104Ns48WJ
pub enum TextModerationService {
    /// 用户昵称检测
    NicknameDetection,
    /// 用户昵称检测_专业版
    NicknameDetectionPro,
    /// 私聊互动内容检测
    ChaDetection,
    /// 私聊互动内容检测_专业版
    ChatDetectionPro,
    /// 公聊评论内容检测
    CommentDetection,
    /// 公聊评论内容检测_专业版
    CommentDetectionPro,
    /// PGC通用物料检测
    PgcDetection,
    /// AIGC类文字检测
    AiArtDetection,
    /// 大语言模型输入文字检测
    LlmQueryModeration,
    /// 大语言模型生成文字检测
    LlmResponseModeration,
    /// 广告法合规检测
    AdComplianceDetection,
    /// 国际业务多语言检测
    CommentMultilingualPro,
    /// URL风险链接检测
    UrlDetection
}

impl TextModerationService {
    pub fn to_string(self) -> String {
        match self {
            TextModerationService::NicknameDetection => "nickname_detection".to_string(),
            TextModerationService::NicknameDetectionPro => "nickname_detection_pro".to_string(),
            TextModerationService::ChaDetection => "chat_detection".to_string(),
            TextModerationService::ChatDetectionPro => "chat_detection_pro".to_string(),
            TextModerationService::CommentDetection => "comment_detection".to_string(),
            TextModerationService::CommentDetectionPro => "comment_detection_pro".to_string(),
            TextModerationService::PgcDetection => "pgc_detection".to_string(),
            TextModerationService::AiArtDetection => "ai_art_detection".to_string(),
            TextModerationService::LlmQueryModeration => "llm_query_moderation".to_string(),
            TextModerationService::LlmResponseModeration => "llm_response_moderation".to_string(),
            TextModerationService::AdComplianceDetection => "ad_compliance_detection".to_string(),
            TextModerationService::CommentMultilingualPro => "comment_multilingual_pro".to_string(),
            TextModerationService::UrlDetection => "url_detection".to_string()
        }
    }
}