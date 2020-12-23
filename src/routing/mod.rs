use std::{error::Error, str::FromStr};

use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

use crate::{general_request::make_https_url_encoded_request, Configuration};

pub enum RoutingError {
    InvalidUrl,
    InvalidService,
    InvalidOptions,
    InvalidQuery,
    InvalidValue,
    NoSegment,
    TooBig,
    NoMatch,
}

fn from_response_to_err(code: u16) -> RoutingError {
    unimplemented!()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoutingResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoutingProfile {
    Driving,
}

impl ToString for RoutingProfile {
    fn to_string(&self) -> String {
        match self {
            RoutingProfile::Driving => "driving".to_string(),
        }
    }
}

impl FromStr for RoutingProfile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "driving" => Ok(RoutingProfile::Driving),
            _ => Err(format!("Can't convert from: {}!", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoutingCallType {
    Directions,
    Matching,
    Matrix,
    Nearest,
    Optimize,
}

impl ToString for RoutingCallType {
    fn to_string(&self) -> String {
        match self {
            RoutingCallType::Directions => "directions".to_string(),
            RoutingCallType::Matching => "matching".to_string(),
            RoutingCallType::Matrix => "matrix".to_string(),
            RoutingCallType::Nearest => "nearest".to_string(),
            RoutingCallType::Optimize => "optimize".to_string(),
        }
    }
}

impl FromStr for RoutingCallType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "directions" => Ok(RoutingCallType::Directions),
            "matching" => Ok(RoutingCallType::Matching),
            "matrix" => Ok(RoutingCallType::Matrix),
            "nearest" => Ok(RoutingCallType::Nearest),
            "optimize" => Ok(RoutingCallType::Optimize),
            _ => Err(format!("Can't convert from: {}!", s)),
        }
    }
}

pub struct RoutingApi<'a> {
    conf: &'a Configuration,
    client: &'a Client<HttpsConnector<HttpConnector>>,
}

impl<'a> RoutingApi<'a> {
    pub fn new(conf: &'a Configuration, client: &'a Client<HttpsConnector<HttpConnector>>) -> Self {
        RoutingApi { conf, client }
    }

    pub async fn routing(
        &self,
        service: RoutingCallType,
        profile: RoutingProfile,
        coords: Vec<(f32, f32)>,
    ) -> Result<RoutingResponse, Box<dyn Error + Sync>> {
        let params = &[("key", self.conf.get_key())];
        let mut coords_str = String::new();

        for (lat, long) in coords.iter() {
            coords_str.push_str(format!("{},{};", lat, long).as_str());
        }

        let path = format!(
            "/v1/{}/{}/{}",
            service.to_string(),
            profile.to_string(),
            coords_str.as_str()
        );

        make_https_url_encoded_request(
            params,
            self.conf.get_endpoint_str(),
            path.as_str(),
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
    async fn routing_works() {
        /*let key = "pk.0665f8915bc125f09786f3be39cc0ec1";
        let res = ApiClient::new(Configuration::new(
            crate::LocationIQEndpoint::US,
            key.to_string(),
        ))
        .get_geocoding_api()
        .reverse(21.386367, -77.922801, None)
        .await;
        println!("{:?}", res);

        assert!(res.is_ok());*/
    }
}
