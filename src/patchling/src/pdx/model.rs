use serde::{
    de::{
        value::{MapAccessDeserializer, SeqAccessDeserializer},
        Error, MapAccess, SeqAccess, Visitor,
    },
    *,
};
use std::{fmt, fmt::Formatter, marker::PhantomData, sync::Arc};

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PdxRelationType {
    Normal, // this is normally represented with `nil` in Lua, but is in fact stable.
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
}
impl Default for PdxRelationType {
    fn default() -> Self {
        PdxRelationType::Normal
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]

pub enum PdxRelationValue {
    #[serde(rename = "block")]
    Block(PdxBlock),
    #[serde(rename = "val")]
    String(Arc<str>),
    #[serde(rename = "num")]
    Numeric(f64),
    #[serde(rename = "var")]
    Variable(Arc<str>),
    #[serde(rename = "var_expr")]
    VariableExpr(Arc<str>),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct PdxRelation {
    pub tag: Arc<str>,
    #[serde(skip_serializing_if = "is_relation_normal", default)]
    pub relation: PdxRelationType,
    #[serde(flatten)]
    pub value: PdxRelationValue,
}
fn is_relation_normal(relation: &PdxRelationType) -> bool {
    *relation == PdxRelationType::Normal
}

#[derive(Serialize, PartialEq, Clone, Debug)]
#[serde(untagged)] // untagged serialize does what we want
pub enum PdxBlockContent {
    Relation(PdxRelation),
    String(Arc<str>),
}
impl<'de> Deserialize<'de> for PdxBlockContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        deserializer.deserialize_any(PdxBlockVisitor(()))
    }
}

struct PdxBlockVisitor(());
impl<'de> Visitor<'de> for PdxBlockVisitor {
    type Value = PdxBlockContent;
    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("block member")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: Error {
        Ok(PdxBlockContent::String(v.into()))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where A: SeqAccess<'de> {
        Ok(PdxBlockContent::Relation(PdxRelation::deserialize(SeqAccessDeserializer::new(seq))?))
    }
    fn visit_map<A>(self, map: A) -> Result<Self::Value, <A as MapAccess<'de>>::Error>
    where A: MapAccess<'de> {
        Ok(PdxBlockContent::Relation(PdxRelation::deserialize(MapAccessDeserializer::new(map))?))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(transparent)]
pub struct PdxBlock {
    pub contents: Vec<PdxBlockContent>,
}
