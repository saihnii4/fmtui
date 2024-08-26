use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct EntityObject {
    #[serde(rename(deserialize = "#text"))]
    pub text: String,
    pub mbid: String,
}

#[derive(Serialize, Deserialize)]
pub struct DateObject {
    #[serde(rename(deserialize = "#text"))]
    pub text: String,
    pub uts: String,
}

impl Default for DateObject {
    fn default() -> Self {
        DateObject {
            text: String::new(),
            uts: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Size {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Serialize, Deserialize)]
pub struct ImageObject {
    #[serde(rename(deserialize = "#text"))]
    pub text: String,
    pub size: Size,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub album: EntityObject,
    pub artist: EntityObject,
    pub image: Vec<ImageObject>,
    pub name: String,
    pub mbid: String,
    pub streamable: String,
    pub url: String,

    #[serde(default)]
    pub date: DateObject,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}