pub mod reverse;

use std::error::Error;

use crate::Configuration;
use crate::{common::ResponseFormat, geocoding::reverse::ReverseResponse};

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

        let perc_enc = serde_urlencoded::to_string(params).unwrap();

        // endpoint
        let uri = Uri::builder()
            .scheme("https")
            .authority(self.conf.get_endpoint_str())
            .path_and_query(format!("/v1/reverse.php?{}", perc_enc).as_str())
            .build()
            .unwrap();

        // create the request
        let rq: Request<Body> = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .body(Body::empty())
            .unwrap();

        // perform the request & wait for the response
        let resp: Response<Body> = match self.client.request(rq).await {
            Ok(val) => val,
            Err(e) => return Err(Box::new(e)),
        };

        // get all chunks from the response body & store them
        let buf = match hyper::body::to_bytes(resp).await {
            Ok(b) => b,
            Err(e) => return Err(Box::new(e)),
        };

        // parse the response as a json object
        let res: ReverseResponse = match serde_json::from_slice(buf.bytes()) {
            Ok(r) => r,
            Err(e) => return Err(Box::new(e)),
        };

        Ok(res)
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

        assert!(res.is_ok());
    }
}
