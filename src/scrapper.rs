use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

const URL_MAP: &str = "https://www.pollens.fr/load_vigilance_map";

#[derive(serde::Deserialize, Debug)]
struct Map {
    #[serde(rename = "vigilanceMapCounties", deserialize_with = "de_str_as_json")]
    vigilance_map_counties: HashMap<u8, Departement>,
}

/// Désérialise une chaîne de caractère en tant que JSON
fn de_str_as_json<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    deserializer: D,
) -> Result<T, D::Error> {
    use serde::de::Error;
    let str = String::deserialize(deserializer)?;
    serde_json::from_str(&str).map_err(D::Error::custom)
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Departement {
    pub county_name: String,
    pub risk_level: u8,
    pub risks: Vec<Risk>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Risk {
    pub pollen_name: String,
    pub level: u8,
}

pub async fn scrap() -> HashMap<u8, Departement> {
    let reponse: Map = reqwest::get(URL_MAP).await.unwrap().json().await.unwrap();
    reponse.vigilance_map_counties
}
