use std::error::Error;

use crate::Configuration;
use crate::{common::ResponseFormat, geocoding::reverse::ReverseResponse};

use hyper::{body::Buf, client::HttpConnector, Body, Client, Method, Request, Response, Uri};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Deserialize};

pub(crate) async fn make_https_url_encoded_request<C>(
    params: &[(&str, &str)],
    authority: &str,
    path: &str,
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<C, Box<dyn Error + Sync>>
where
    C: DeserializeOwned,
{
    // get the percent encoding repr for the arguments
    let perc_enc = serde_urlencoded::to_string(params).unwrap();

    // endpoint
    let uri = Uri::builder()
        .scheme("https")
        .authority(authority)
        .path_and_query(format!("{}?{}", path, perc_enc).as_str())
        .build()
        .unwrap();
    // create the request
    let rq: Request<Body> = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    // perform the request & wait for the response
    let resp: Response<Body> = match client.request(rq).await {
        Ok(val) => val,
        Err(e) => return Err(Box::new(e)),
    };

    // get all chunks from the response body & store them
    let buf = match hyper::body::to_bytes(resp).await {
        Ok(b) => b,
        Err(e) => return Err(Box::new(e)),
    };

    // parse the response as a json object
    let res = match serde_json::from_slice(buf.bytes()) {
        Ok(r) => r,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(res)
}
