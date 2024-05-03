use reqwest::{Client, ClientBuilder};

pub struct GreenClient {
    pub access_key_id: String,
    pub access_key_secret: String,
    /// 接入点（根据接入地域选择）
    pub endpoint: String,
    /// 请求超时时间，单位毫秒（ms）
    pub timeout: u64,
    /// reqwest 客户端
    pub client: Client
}

impl GreenClient {
    pub fn builder() -> GreenClientBuilder {
        GreenClientBuilder {
            access_key_id: None,
            access_key_secret: None,
            region: Region::Shanghai,
            network_type: NetworkType::Internal,
            timeout: 6000,
            https: true,
            danger_accept_invalid_certs: false
        }
    }
}

pub struct GreenClientBuilder {
    access_key_id: Option<String>,
    access_key_secret: Option<String>,
    region: Region,
    network_type: NetworkType,
    timeout: u64,
    https: bool,
    danger_accept_invalid_certs: bool
}

impl GreenClientBuilder {
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

    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn https(mut self, https: bool) -> Self {
        self.https = https;
        self
    }

    pub fn danger_accept_invalid_certs(mut self, danger_accept_invalid_certs: bool) -> Self {
        self.danger_accept_invalid_certs = danger_accept_invalid_certs;
        self
    }

    pub fn build(self) -> GreenClient {
        let access_key_id = self.access_key_id.expect("access_key_id is not set.");
        let access_key_secret = self.access_key_secret.expect("access_key_secret is not set.");
        let endpoint = gen_endpoint(&self.region, &self.network_type, self.https);

        let green_cpi_client = GreenClient {
            access_key_id,
            access_key_secret,
            endpoint,
            timeout: self.timeout,
            client: ClientBuilder::new()
                .danger_accept_invalid_certs(self.danger_accept_invalid_certs)
                .build()
                .unwrap()
        };

        green_cpi_client
    }
}

pub enum NetworkType {
    Internal,
    External,
}

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

/// 生成 endpoint
fn gen_endpoint(region: &Region, network_type: &NetworkType, https: bool) -> String {
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

    let protocol = if https { "https" } else { "http" };
    format!("{}://{}.{}.aliyuncs.com", protocol, network, region)
}

