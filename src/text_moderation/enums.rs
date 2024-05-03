pub enum TextModerationService {
    NicknameDetection,
    ChaDetection,
    CommentDetection,
    AiArtDetection,
    AdComplianceDetection,
    PgcDetection,
}

impl TextModerationService {
    pub fn to_string(self) -> String {
        match self {
            TextModerationService::NicknameDetection => "nickname_detection".to_string(),
            TextModerationService::ChaDetection => "chat_detection".to_string(),
            TextModerationService::CommentDetection => "comment_detection".to_string(),
            TextModerationService::AiArtDetection => "ai_art_detection".to_string(),
            TextModerationService::AdComplianceDetection => "ad_compliance_detection".to_string(),
            TextModerationService::PgcDetection => "pgc_detection".to_string()
        }
    }
}