use serde::de::{DeserializeOwned, Error, Unexpected};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

const URL_MAP: &str = "https://www.pollens.fr/load_vigilance_map";

#[derive(serde::Deserialize, Debug)]
struct Map {
    #[serde(rename = "vigilanceMapCounties", deserialize_with = "de_str_as_json")]
    vigilance_map_counties: HashMap<DepartementNum, Departement>,
}

/// Désérialise une chaîne de caractère en tant que JSON
fn de_str_as_json<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    deserializer: D,
) -> Result<T, D::Error> {
    let str = std::borrow::Cow::<str>::deserialize(deserializer)?;
    serde_json::from_str(str.as_ref()).map_err(D::Error::custom)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DepartementNum(pub u8);

impl<'de> Deserialize<'de> for DepartementNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let num_str = <&str>::deserialize(deserializer)?;
        match num_str.trim_start_matches('0').parse::<u8>() {
            Ok(num) => Ok(Self(num)),
            Err(_) => Err(D::Error::invalid_value(
                Unexpected::Str(num_str),
                &"a valid french county number",
            )),
        }
    }
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

pub async fn scrap() -> HashMap<DepartementNum, Departement> {
    let reponse: Map = reqwest::get(URL_MAP).await.unwrap().json().await.unwrap();
    reponse.vigilance_map_counties
}
