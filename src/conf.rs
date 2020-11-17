use hyper::Uri;

use crate::common::LocationIQEndpoint;

pub struct Configuration {
    endpoint : LocationIQEndpoint,
    api_key : String,
}

impl Configuration {
    /// Returns a new configuration given the desired server & api access key
    pub fn new(endpoint: LocationIQEndpoint, api_key : String) -> Self {
        Configuration {
            endpoint,
            api_key
        }
    }

    pub fn get_key(&self) -> &str {
        &self.api_key
    }

    pub fn get_endpoint_str(&self) -> &str {
        &self.endpoint.as_str()
    }
}
