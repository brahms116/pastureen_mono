
pub enum HtmxUrl {
    Get(String),
    Post(String),
    Put(String),
    Delete(String),
}

#[derive(Default)]
pub struct DerivedHtmxUrls {
    pub get: Option<String>,
    pub post: Option<String>,
    pub put: Option<String>,
    pub delete: Option<String>,
}

impl From<HtmxUrl> for DerivedHtmxUrls {
    fn from(url: HtmxUrl) -> Self {
        match url {
            HtmxUrl::Get(url) => Self {
                get: Some(url),
                post: None,
                put: None,
                delete: None,
            },
            HtmxUrl::Post(url) => Self {
                get: None,
                post: Some(url),
                put: None,
                delete: None,
            },
            HtmxUrl::Put(url) => Self {
                get: None,
                post: None,
                put: Some(url),
                delete: None,
            },
            HtmxUrl::Delete(url) => Self {
                get: None,
                post: None,
                put: None,
                delete: Some(url),
            },
        }
    }
}

pub struct HtmxOptions {
    pub target: Option<String>,
    pub url: Option<HtmxUrl>,
    pub swap: Option<String>,
    pub trigger: Option<String>,
    pub indicator: Option<String>,
}
