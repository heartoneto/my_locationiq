mod client;
mod common;
mod conf;
mod general_request;
mod geocoding;
mod osm_types;
mod routing;

pub use client::ApiClient;
pub use common::LocationIQEndpoint;
pub use conf::Configuration;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
