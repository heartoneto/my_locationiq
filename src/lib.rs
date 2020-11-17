mod common;
mod osm_types;
mod conf;
mod general_request;
mod client;
mod geocoding;

pub use common::LocationIQEndpoint;
pub use conf::Configuration;
pub use client::ApiClient;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
