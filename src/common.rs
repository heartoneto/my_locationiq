use hyper::Uri;

// LocationIQ US server endpoint
static US_SERVER_ADDR: &str = "us1.locationiq.com";

// LocationIQ EU server endpoint
static EU_SERVER_ADDR: &str = "eu1.locationiq.com";

/// Represents a server endpoint
pub enum LocationIQEndpoint {
    US,
    EU,
}

impl LocationIQEndpoint {
    pub fn as_str(&self) -> &str {
        match self {
            LocationIQEndpoint::US => US_SERVER_ADDR,
            LocationIQEndpoint::EU => EU_SERVER_ADDR,
        }
    }
}

pub enum ResponseFormat {
    JSON,
    XML,
}

impl std::string::ToString for ResponseFormat {
    fn to_string(&self) -> String {
        match self {
            ResponseFormat::JSON => "json".to_string(),
            ResponseFormat::XML => "xml1.1".to_string(),
        }
    }
}
