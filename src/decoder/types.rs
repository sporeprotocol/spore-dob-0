use alloc::string::String;
use alloc::vec::Vec;

use crate::schema::dob_0::TraitsBase;

#[repr(u64)]
pub enum Error {
    ParseInvalidArgCount = 1,
    ParseInvalidSporeDNA,
    ParseInvalidTraitsBase,

    DecodeInsufficientSporeDNA,
    DecodeUnexpectedDNASegment,
    DecodeEmptyStringVecTrait,
    DecodeEmptyNumberVecTrait,
    DecodeEmptyFloatVecTrait,
    DecodeInvalidNumberRangeTrait,
    DecodeInvalidFloatRangeTrait,
}

#[derive(serde::Serialize)]
pub enum ParsedTrait {
    String(String),
    Number(u64),
    Float(f64),
}

#[derive(serde::Serialize, Default)]
pub struct ParsedDNA {
    pub name: String,
    pub traits: Vec<ParsedTrait>,
}

pub struct Parameters {
    pub spore_dna: Vec<u8>,
    pub traits_base: TraitsBase,
}
