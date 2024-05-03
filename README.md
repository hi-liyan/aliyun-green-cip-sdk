# Aliyun Green Cip Sdk

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

阿里云内容安全服务的Rust SDK，目前实现了`文本审核增强版API`和`图片审核增强版API`。

## 支持

- [X] 支持文本审核增强版API
- [X] 支持图片审核增强版API
- [ ] 语音审核增强版API
- [ ] 视频文件审核增强版API
- [ ] 文档审核增强版API
- [ ] URL风险同步检测API
- [ ] ...

## 用法

### 添加依赖

```toml
[dependencies]
aliyun-green-cip-sdk = { git = "https://github.com/hi-liyan/aliyun-green-cip-sdk.git" }
```

### 构建客户端

```rust
fn main() {
    let client = GreenClient::builder()
        // 阿里云 ACCESS_KEY_ID 必填
        .access_key_id("access_key_id".to_string())
        // 阿里云 ACCESS_KEY_SECRET 必填
        .access_key_secret("access_key_secret".to_string())
        // 区域 默认华东2（上海）
        .region(Region::Shanghai)
        // 网络类型 默认内网
        .network_type(NetworkType::Internal)
        // 请求超时时间 从请求发起开始直到响应结束 默认6000
        .timeout(6000)
        // 是否开启 HTTPS 默认开启
        .https(true)
        // reqwest 参数 是否忽略证书
        .danger_accept_invalid_certs(false)
        .build();
}
```

### 文本审核

```rust
async fn main() {
    // 构建请求参数
    let request_params = TextModerationRequest::builder()
        .service(Some(TextModerationService::NicknameDetection.to_string()))
        .content(Some("测试文本".to_string()))
        .account_id(Some("1001".to_string()))
        .device_id(Some("ios_0".to_string()))
        .build();

    let response = client.text_moderation(request_params).await;
    println!("response: {:?}", response);
}
```

### 图片审核

```rust
async fn main() {
    // 构建请求参数
    let request_params = ImageModerationRequest::builder()
        .service(Some(ImageModerationService::BaselineCheck.to_string()))
        .image_url(Some("".to_string()))
        .data_id(Some("1001".to_string()))
        .build();

    let response = client.image_moderation(request_params).await;
    println!("response: {:?}", response);
}
```
