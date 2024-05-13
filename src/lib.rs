use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type DimensionID = String;
type RelationID = String;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct JSONStat {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Href>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<Extension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Role {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Href>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Link {
    RelationObject(HashMap<RelationID, Vec<LinkProperties>>),
    RelationArray(Vec<LinkProperties>),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Dimension(pub HashMap<DimensionID, DimensionObject>);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DimensionObject {
    pub category: Category,
    pub label: Label,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Index {
    Array(Vec<String>),
    Object(HashMap<String, usize>),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Category {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Index>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child: Option<Child>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Coordinates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Label {
    String(String),
    Object(HashMap<String, String>),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Child(pub HashMap<String, Vec<String>>);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Coordinates(pub HashMap<String, (f64, f64)>);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Unit(pub HashMap<String, UnitProperties>);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct UnitProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimals: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Position {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Source(pub String);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Extension(pub HashMap<String, Value>);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Href(pub String);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Note(pub Vec<String>);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Error(pub Vec<HashMap<String, Value>>);