use std::env;
use reqwest::{Client, ClientBuilder};
use struct_builder::Builder;

mod common;
#[cfg(test)]
mod test;
mod text_audit;

/// # 内容安全客户端
pub struct GreenCpiClient {
    pub access_key_id: String,
    pub access_key_secret: String,
    /// 接入点（根据接入地域选择）
    pub endpoint: String,
    /// 连接时超时时间，单位毫秒（ms）
    pub connect_timeout: u32,
    /// 读取时超时时间，单位毫秒（ms）。
    pub read_timeout: u32,
    /// reqwest 客户端
    pub client: Client
}

impl GreenCpiClient {
    pub fn builder() -> GreenCpiClientBuilder {
        GreenCpiClientBuilder {
            access_key_id: None,
            access_key_secret: None,
            region: Region::Shanghai,
            network_type: NetworkType::Internal,
            connect_timeout: 6000,
            read_timeout: 3000,
            danger_accept_invalid_certs: false
        }
    }
}


/// # 内容安全客户端构造器
struct GreenCpiClientBuilder {
    access_key_id: Option<String>,
    access_key_secret: Option<String>,
    region: Region,
    network_type: NetworkType,
    connect_timeout: u32,
    read_timeout: u32,
    danger_accept_invalid_certs: bool
}

impl GreenCpiClientBuilder {
    pub fn access_key_id(mut self, access_key_id: String) -> Self {
        self.access_key_id = Some(access_key_id);
        self
    }

    pub fn access_key_secret(mut self, access_key_secret: String) -> Self {
        self.access_key_secret = Some(access_key_secret);
        self
    }

    pub fn region(mut self, region: Region) -> Self {
        self.region = region;
        self
    }

    pub fn network_type(mut self, network_type: NetworkType) -> Self {
        self.network_type = network_type;
        self
    }

    pub fn connect_timeout(mut self, connect_timeout: u32) -> Self {
        self.connect_timeout = connect_timeout;
        self
    }

    pub fn read_timeout(mut self, read_timeout: u32) -> Self {
        self.read_timeout = read_timeout;
        self
    }

    pub fn danger_accept_invalid_certs(mut self, danger_accept_invalid_certs: bool) -> Self {
        self.danger_accept_invalid_certs = danger_accept_invalid_certs;
        self
    }

    pub fn build(self) -> GreenCpiClient {
        let access_key_id = self.access_key_id
            .unwrap_or(env::var("ALIBABA_CLOUD_ACCESS_KEY_ID").expect("读取不到 ALIBABA_CLOUD_ACCESS_KEY_ID 环境变量"));

        let access_key_secret = self.access_key_secret
            .unwrap_or(env::var("ALIBABA_CLOUD_ACCESS_KEY_SECRET").expect("读取不到 ALIBABA_CLOUD_ACCESS_KEY_SECRET 环境变量"));

        let endpoint = get_endpoint(&self.region, &self.network_type);

        let green_cpi_client = GreenCpiClient {
            access_key_id,
            access_key_secret,
            endpoint,
            connect_timeout: self.connect_timeout,
            read_timeout: self.read_timeout,
            client: ClientBuilder::new()
                .danger_accept_invalid_certs(self.danger_accept_invalid_certs)
                .build()
                .unwrap()
        };

        green_cpi_client
    }
}

/// # 访问凭据
#[derive(Debug, Clone, Builder)]
pub struct Credential {
    access_key_id: Option<String>,
    access_key_secret: Option<String>
}

impl Credential {
    /// 获取 ALIBABA_CLOUD_ACCESS_KEY_ID
    pub fn access_key_id(&self) -> Option<&String> {
        self.access_key_id.as_ref()
    }

    /// ALIBABA_CLOUD_ACCESS_KEY_SECRET
    pub fn access_key_secret(&self) -> Option<&String> {
        self.access_key_secret.as_ref()
    }
}

/// # 接入点网络类型
///
/// - Internal 内网
/// - External 外网
enum NetworkType {
    Internal,
    External,
}


/// # Rest Api 接入地域
///
/// 参考文档：
/// - https://help.aliyun.com/document_detail/433945.html?spm=a2c4g.434034.0.0.50d959c5WDPDFP
#[derive(Clone)]
pub enum Region {
    /// 华东2（上海）
    Shanghai,
    /// 华北2（北京）
    Beijing,
    /// 华东1（杭州）
    Hangzhou,
    /// 华南1（深圳）
    Shenzhen,
    /// 西南1（成都）
    Chengdu,
    /// 新加坡
    ApSoutheast
}

/// 获取接入点
fn get_endpoint(region: &Region, network_type: &NetworkType) -> String {
    let network = match network_type {
        NetworkType::Internal => "green-cip-vpc",
        NetworkType::External => "green-cip"
    };

    let region = match region {
        Region::Shanghai => "cn-shanghai",
        Region::Beijing => "cn-beijing",
        Region::Hangzhou => "cn-hangzhou",
        Region::Shenzhen => "cn-shenzhen",
        Region::Chengdu => "cn-chengdu",
        Region::ApSoutheast => "ap-southeast"
    };

    format!("{}.{}.aliyuncs.com", network, region)
}



