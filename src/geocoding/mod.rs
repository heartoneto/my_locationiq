pub mod reverse;

use std::error::Error;

use crate::{common::ResponseFormat, geocoding::reverse::ReverseResponse};
use crate::{general_request::make_https_url_encoded_request, Configuration};

use hyper::{body::Buf, client::HttpConnector, Body, Client, Method, Request, Response, Uri};
use hyper_tls::HttpsConnector;

pub struct GeoCodingApi<'a> {
    conf: &'a Configuration,
    client: &'a Client<HttpsConnector<HttpConnector>>,
}

impl<'a> GeoCodingApi<'a> {
    pub fn new(conf: &'a Configuration, client: &'a Client<HttpsConnector<HttpConnector>>) -> Self {
        GeoCodingApi { conf, client }
    }

    /// Format defaults to json if not given
    pub async fn reverse(
        &self,
        lat: f32,
        lon: f32,
        format: Option<ResponseFormat>,
    ) -> Result<ReverseResponse, Box<dyn Error + Sync>> {
        let end_format = format.unwrap_or(ResponseFormat::JSON);

        // request body
        let params = &[
            ("key", self.conf.get_key()),
            ("lat", &lat.to_string()),
            ("lon", &lon.to_string()),
            ("format", &end_format.to_string()),
        ];

        make_https_url_encoded_request(
            params,
            self.conf.get_endpoint_str(),
            "/v1/reverse.php",
            self.client,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::ApiClient;

    use super::*;

    #[tokio::test]
    async fn reverse_geo_works() {
        let key = "pk.0665f8915bc125f09786f3be39cc0ec1";
        let res = ApiClient::new(Configuration::new(
            crate::LocationIQEndpoint::US,
            key.to_string(),
        ))
        .get_geocoding_api()
        .reverse(21.386367, -77.922801, None)
        .await;
        println!("{:?}", res);

        assert!(false);
    }
}
