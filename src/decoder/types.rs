use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
use serde_json::Value;

#[repr(u64)]
#[cfg_attr(test, derive(Debug))]
pub enum Error {
    ParseInvalidArgCount = 1,
    ParseInvalidSporeDNA,
    ParseInvalidTraitsBase,

    SchemaInsufficientElements,
    SchemaInvalidName,
    SchemaInvalidType,
    SchemaTypeMismatch,
    SchemaInvalidOffset,
    SchemaInvalidLen,
    SchemaInvalidPattern,
    SchemaPatternMismatch,
    SchemaInvalidArgs,
    SchemaInvalidArgsElement,

    DecodeInsufficientSporeDNA,
    DecodeUnexpectedDNASegment,
    DecodeArgsTypeMismatch,
    DecodeMissingRangeArgs,
    DecodeInvalidRangeArgs,
    DecodeMissingOptionArgs,
    DecodeInvalidOptionArgs,
    DecodeBadUTF8Format,
}

#[derive(serde::Serialize)]
pub enum ParsedTrait {
    String(String),
    Number(u64),
}

#[derive(serde::Serialize, Default)]
pub struct ParsedDNA {
    pub name: String,
    pub traits: Vec<ParsedTrait>,
}

#[cfg_attr(test, derive(serde::Serialize))]
pub struct Parameters {
    pub spore_dna: Vec<u8>,
    pub traits_base: Vec<TraitSchema>,
}

#[cfg_attr(test, derive(serde::Serialize, Clone))]
#[derive(serde::Deserialize, PartialEq, Eq)]
pub enum ArgsType {
    String,
    Number,
}

#[cfg_attr(test, derive(serde::Serialize, Clone))]
#[derive(serde::Deserialize)]
pub enum Pattern {
    Options,
    Range,
    Raw,
    Utf8,
}

#[cfg_attr(test, derive(serde::Serialize, Clone))]
#[derive(serde::Deserialize)]
pub struct TraitSchema {
    pub name: String,
    pub type_: ArgsType,
    pub offset: u64,
    pub len: u64,
    pub pattern: Pattern,
    pub args: Option<Vec<String>>,
}

#[cfg(test)]
impl TraitSchema {
    #[allow(dead_code)]
    pub fn new(
        name: &str,
        type_: ArgsType,
        offset: u64,
        len: u64,
        pattern: Pattern,
        args: Option<Vec<&str>>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            type_,
            offset,
            len,
            pattern,
            args: args.map(|v| v.into_iter().map(ToOwned::to_owned).collect()),
        }
    }

    #[allow(dead_code)]
    pub fn encode(&self) -> Vec<Value> {
        let mut values = vec![
            Value::String(self.name.clone()),
            Value::String(match self.type_ {
                ArgsType::String => "string".to_owned(),
                ArgsType::Number => "number".to_owned(),
            }),
            Value::Number(self.offset.into()),
            Value::Number(self.len.into()),
            Value::String(match self.pattern {
                Pattern::Options => "options".to_owned(),
                Pattern::Range => "range".to_owned(),
                Pattern::Raw => "raw".to_owned(),
                Pattern::Utf8 => "utf8".to_owned(),
            }),
        ];
        if let Some(args) = &self.args {
            values.push(Value::Array(match self.type_ {
                ArgsType::String => args.iter().map(|v| Value::String(v.clone())).collect(),
                ArgsType::Number => args
                    .iter()
                    .map(|v| Value::Number(v.parse().unwrap()))
                    .collect(),
            }));
        }
        values
    }
}

pub fn decode_trait_schema(traits_pool: Vec<Vec<Value>>) -> Result<Vec<TraitSchema>, Error> {
    let traits_base = traits_pool
        .into_iter()
        .map(|schema| {
            if schema.len() < 5 {
                return Err(Error::SchemaInsufficientElements);
            }
            let name = schema[0].as_str().ok_or(Error::SchemaInvalidName)?;
            let type_ = match schema[1].as_str().ok_or(Error::SchemaInvalidType)? {
                "string" => ArgsType::String,
                "number" => ArgsType::Number,
                _ => return Err(Error::SchemaTypeMismatch),
            };
            let offset = schema[2].as_u64().ok_or(Error::SchemaInvalidOffset)?;
            let len = schema[3].as_u64().ok_or(Error::SchemaInvalidLen)?;
            let pattern_str = schema[4].as_str().ok_or(Error::SchemaInvalidPattern)?;
            let pattern = match (pattern_str, &type_) {
                ("options", _) => Pattern::Options,
                ("raw", _) => Pattern::Raw,
                ("utf8", ArgsType::String) => Pattern::Utf8,
                ("range", ArgsType::Number) => Pattern::Range,
                _ => return Err(Error::SchemaPatternMismatch),
            };
            let args = if let Some(args) = schema.get(5) {
                let args = args
                    .as_array()
                    .ok_or(Error::SchemaInvalidArgs)?
                    .iter()
                    .map(|value| {
                        if value.is_string() {
                            value
                                .as_str()
                                .ok_or(Error::SchemaInvalidArgsElement)
                                .map(ToOwned::to_owned)
                        } else {
                            Ok(format!(
                                "{}",
                                value.as_u64().ok_or(Error::SchemaInvalidArgsElement)?
                            ))
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Some(args)
            } else {
                None
            };
            Ok(TraitSchema {
                name: name.to_owned(),
                type_,
                offset,
                len,
                pattern,
                args,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(traits_base)
}
