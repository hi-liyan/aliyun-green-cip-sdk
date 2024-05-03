
pub enum ImageModerationService {
    BaselineCheck,
    BaselineCheckPro,
    BaselineCheckCb,
    TonalityImprove,
    AigcCheck,
    AigcCheckCb,
    ProfilePhotoCheck,
    AdvertisingCheck,
    LiveStreamCheck
}

impl ImageModerationService {
    pub fn to_string(self) -> String {
        match self {
            ImageModerationService::BaselineCheck => "baselineCheck".to_string(),
            ImageModerationService::BaselineCheckPro => "baselineCheck_pro".to_string(),
            ImageModerationService::BaselineCheckCb => "baselineCheck_cb".to_string(),
            ImageModerationService::TonalityImprove => "tonalityImprove".to_string(),
            ImageModerationService::AigcCheck => "aigcCheck".to_string(),
            ImageModerationService::AigcCheckCb => "aigcCheck_cb".to_string(),
            ImageModerationService::ProfilePhotoCheck => "profilePhotoCheck".to_string(),
            ImageModerationService::AdvertisingCheck => "advertisingCheck".to_string(),
            ImageModerationService::LiveStreamCheck => "liveStreamCheck".to_string()
        }
    }
}