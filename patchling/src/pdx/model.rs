use serde::{
    de::{
        value::{MapAccessDeserializer, SeqAccessDeserializer},
        Error, MapAccess, SeqAccess, Visitor,
    },
    *,
};
use std::{borrow::Cow, fmt, fmt::Formatter, marker::PhantomData};

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PdxRelationType {
    Normal,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
}
impl Default for PdxRelationType {
    fn default() -> Self {
        PdxRelationType::Normal
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub enum PdxRelationValue<'a> {
    #[serde(rename = "block")]
    Block(PdxBlock<'a>),
    #[serde(rename = "val")]
    String(Cow<'a, str>),
    #[serde(rename = "num")]
    Numeric(f64),
    #[serde(rename = "var")]
    Variable(Cow<'a, str>),
    #[serde(rename = "var_expr")]
    VariableExpr(Cow<'a, str>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PdxRelation<'a> {
    pub tag: Cow<'a, str>,
    #[serde(skip_serializing_if = "is_relation_normal", default)]
    pub relation: PdxRelationType,
    #[serde(flatten)]
    pub value: PdxRelationValue<'a>,
}
fn is_relation_normal(relation: &PdxRelationType) -> bool {
    *relation == PdxRelationType::Normal
}

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)] // untagged serialize does what we want
pub enum PdxBlockContent<'a> {
    Relation(PdxRelation<'a>),
    String(Cow<'a, str>),
}
impl<'a, 'de> Deserialize<'de> for PdxBlockContent<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        deserializer.deserialize_any(PdxBlockVisitor(PhantomData))
    }
}

struct PdxBlockVisitor<'a>(PhantomData<&'a ()>);
impl<'a, 'de> Visitor<'de> for PdxBlockVisitor<'a> {
    type Value = PdxBlockContent<'a>;
    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("block member")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: Error {
        Ok(PdxBlockContent::String(Cow::Owned(v.to_string())))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where E: Error {
        Ok(PdxBlockContent::String(Cow::Owned(v)))
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct PdxBlock<'a> {
    pub contents: Vec<PdxBlockContent<'a>>,
}
