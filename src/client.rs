use std::rc::Rc;

use hyper::{Body, Client};
use hyper_tls::HttpsConnector;

use crate::{conf::Configuration, geocoding::GeoCodingApi};

pub struct ApiClient {
    conf: Configuration,
    client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl ApiClient {
    pub fn new(conf: Configuration) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);

        ApiClient { conf, client }
    }

    pub fn get_geocoding_api(&self) -> GeoCodingApi {
        GeoCodingApi::new(&self.conf, &self.client)
    }
}
