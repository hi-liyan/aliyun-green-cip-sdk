use chrono::Utc;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use url::form_urlencoded;

/// 获取 UTC 时间
pub fn get_utc() -> String {
    let now = Utc::now();
    let utc = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    utc
}

/// HMAC SHA1 加密
pub fn hmac_sha1(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut hmac: Hmac<Sha1> =
        Hmac::<Sha1>::new_from_slice(key).expect("HMAC can take key of any size");
    hmac.update(message);
    hmac.finalize().into_bytes().to_vec()
}

/// URL 编码
fn encode_param(param: &str) -> String {
    form_urlencoded::byte_serialize(param.as_bytes()).collect::<String>()
}

/// 生成 Canonical Query String
pub fn generate_canonical_query_string(params: &[(&str, &str)]) -> String {
    let mut encoded_params: Vec<String> = params
        .iter()
        .map(|(key, value)| format!("{}={}", encode_param(key), encode_param(value)))
        .collect();
    encoded_params.sort();
    encoded_params.join("&")
}

/// 生成代签名字符串
pub fn generate_string_to_sign(http_method: &str, canonical_query_string: &str) -> String {
    format!("{}&%2F&{}", http_method, encode_param(canonical_query_string))
}

