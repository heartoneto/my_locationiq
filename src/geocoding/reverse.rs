use serde::{Deserialize, Serialize};

use crate::osm_types::OsmType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    road: Option<String>,
    suburb: Option<String>,
    county: Option<String>,
    region: Option<String>,
    state: String,
    postcode: String,
    country: String,
    country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReverseResponse {
    place_id: String,
    licence: String,
    osm_type: OsmType,
    osm_id: String,
    lat: String,
    lon: String,
    display_name: String,
    address: Option<Address>,

    boundingbox: [String; 4],
    importance: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::ReverseResponse;

    #[test]
    fn can_parse_reverse_response() {
        let json_response = r#"{
            "place_id": "26693344",
            "licence": "© LocationIQ.com CC BY 4.0, Data © OpenStreetMap contributors, ODbL 1.0",
            "osm_type": "node",
            "osm_id": "2525193585",
            "lat": "-37.870662",
            "lon": "144.9803321",
            "display_name": "Imbiss 25, Blessington Street, St Kilda, City of Port Phillip, Greater Melbourne, Victoria, 3182, Australia",
            "address": {
                "cafe": "Imbiss 25",
                "road": "Blessington Street",
                "suburb": "St Kilda",
                "county": "City of Port Phillip",
                "region": "Greater Melbourne",
                "state": "Victoria",
                "postcode": "3182",
                "country": "Australia",
                "country_code": "au"
            },
            "boundingbox": [
                "-37.870762",
                "-37.870562",
                "144.9802321",
                "144.9804321"
            ]
        }"#;

        let rev_response: Result<ReverseResponse, _> = serde_json::de::from_str(json_response);
        println!("{:?}", rev_response);
        assert!(rev_response.is_ok());
    }
}
