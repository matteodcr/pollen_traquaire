use std::collections::HashMap;

const URL_MAP: &str = "https://www.pollens.fr/load_vigilance_map";

#[derive(serde::Deserialize, Debug)]
struct Map {
    #[serde(rename = "vigilanceMapCounties")]
    vigilance_map_counties: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Departement {
    pub county_name: String,
    pub risk_level: u8,
    pub risks: Vec<Risk>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Risk {
    pub pollen_name: String,
    pub level: u8,
}

pub async fn scrap() -> HashMap<u8, Departement> {
    let reponse: Map = reqwest::get(URL_MAP).await.unwrap().json().await.unwrap();
    serde_json::from_str(&reponse.vigilance_map_counties).unwrap()
}
