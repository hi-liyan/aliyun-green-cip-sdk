use base64::Engine;
use base64::engine::general_purpose;
use crate::client::GreenClient;
use crate::utils::{generate_canonical_query_string, generate_string_to_sign, hmac_sha1};

impl GreenClient {
    /// 生成签名
    pub fn generate_signature(&self, http_method: &str, query_params: &[(&str, &str)]) -> String {
        let canonical_query_string = generate_canonical_query_string(query_params);
        let string_to_sign = generate_string_to_sign(http_method, &canonical_query_string);
        let signature = hmac_sha1(format!("{}&", self.access_key_secret).as_bytes(), string_to_sign.as_bytes());
        general_purpose::STANDARD.encode(signature)
    }
}
