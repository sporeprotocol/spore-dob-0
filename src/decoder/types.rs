use alloc::string::String;
use alloc::vec::Vec;

use crate::schema::dob_721::TraitsBase;

#[repr(u64)]
pub enum Error {
    ParseInvalidArgCount = 1,
    ParseInvalidSporeDNA,
    ParseInvalidBlockNumber,
    ParseInvalidTraitsBase,

    DecodeInsufficientSporeDNA,
    DecodeUnexpectedDNASegment,
    DecodeEmptyStringVecTrait,
    DecodeEmptyNumberVecTrait,
    DecodeEmptyFloatVecTrait,
    DecodeEmptyMutantVecTrait,
    DecodeInvalidNumberRangeTrait,
    DecodeInvalidFloatRangeTrait,
    DecodeInvalidMutantRangeTrait,
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
    pub block_number: u64,
    pub traits_base: TraitsBase,
}
