use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::CStr;
use molecule::prelude::Entity;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

use crate::schema::dob_721::{TraitPoolUnion, TraitsBase};

pub mod types;
use types::{Error, Parameters, ParsedDNA, ParsedTrait};

// example:
// argv[1] = efc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d (hexed DNA string in Spore)
// argv[2] = 1250945 (block number while minting)
// argv[3] = d48869363ff41a103b131a29f43...d7be6eeaf513c2c3ae056b9b8c2e1 (traits config in Cluster)
pub fn dobs_parse_parameters(argc: u64, argv: *const *const i8) -> Result<Parameters, Error> {
    if argc != 4 {
        return Err(Error::ParseInvalidArgCount);
    }
    let mut params = Vec::new();
    for i in 1..argc {
        let argn = unsafe { CStr::from_ptr(argv.add(i as usize).read()) };
        params.push(argn.to_bytes().to_vec());
    }

    let spore_dna = {
        let value = &params[0];
        if value.is_empty() || value.len() % 2 != 0 {
            return Err(Error::ParseInvalidSporeDNA);
        }
        let mut dna = Vec::with_capacity(value.len() / 2);
        faster_hex::hex_decode(&value, &mut dna).map_err(|_| Error::ParseInvalidSporeDNA)?;
        dna
    };
    let block_number = {
        let value = String::from_utf8_lossy(&params[1]);
        u64::from_str_radix(&value, 10).map_err(|_| Error::ParseInvalidBlockNumber)?
    };
    let traits_base = {
        let value = &params[2];
        if value.len() % 2 != 0 {
            return Err(Error::ParseInvalidSporeDNA);
        }
        let mut traits = Vec::with_capacity(value.len() / 2);
        faster_hex::hex_decode(&value, &mut traits).map_err(|_| Error::ParseInvalidTraitsBase)?;
        TraitsBase::from_compatible_slice(&traits).map_err(|_| Error::ParseInvalidTraitsBase)?
    };
    Ok(Parameters {
        spore_dna,
        block_number,
        traits_base,
    })
}

pub fn dobs_decode(parameters: Parameters) -> Result<Vec<u8>, Error> {
    let Parameters {
        mut spore_dna,
        block_number,
        traits_base,
    } = parameters;

    let mut rng = SmallRng::seed_from_u64(block_number);
    let mut result = Vec::new();
    for schema_base in traits_base.into_iter() {
        let mut parsed_dna = ParsedDNA::default();
        parsed_dna.name = schema_base.name().into();
        for schema in schema_base.schema_pool().into_iter() {
            let byte_length = schema.byte_length().into();
            let mut dna_segment = Vec::new();
            for _ in 0..byte_length {
                if spore_dna.is_empty() {
                    return Err(Error::DecodeInsufficientSporeDNA);
                }
                dna_segment.push(spore_dna.remove(0));
            }
            let offset = match dna_segment.len() {
                1 => dna_segment[0] as u64,
                2 => u16::from_le_bytes(dna_segment.try_into().unwrap()) as u64,
                4 => u32::from_be_bytes(dna_segment.try_into().unwrap()) as u64,
                8 => u64::from_le_bytes(dna_segment.try_into().unwrap()),
                _ => return Err(Error::DecodeUnexpectedDNASegment),
            };
            let Some(trait_pool) = schema.trait_pool().to_opt() else {
                parsed_dna.traits.push(ParsedTrait::Number(offset));
                continue;
            };
            match trait_pool.to_enum() {
                TraitPoolUnion::StringVec(strings) => {
                    if strings.is_empty() {
                        return Err(Error::DecodeEmptyStringVecTrait);
                    }
                    let offset = offset as usize % strings.len();
                    let value = strings.get_unchecked(offset).into();
                    parsed_dna.traits.push(ParsedTrait::String(value));
                }
                TraitPoolUnion::NumberVec(numbers) => {
                    if numbers.is_empty() {
                        return Err(Error::DecodeEmptyNumberVecTrait);
                    }
                    let offset = offset as usize % numbers.len();
                    let value = numbers.get_unchecked(offset);
                    parsed_dna.traits.push(ParsedTrait::Number(value.into()));
                }
                TraitPoolUnion::FloatVec(floats) => {
                    if floats.numerator_vec().is_empty() {
                        return Err(Error::DecodeEmptyFloatVecTrait);
                    }
                    let offset = offset as usize % floats.numerator_vec().len();
                    let numerator: u64 = floats.numerator_vec().get_unchecked(offset).into();
                    let denominator: u64 = floats.denominator().into();
                    parsed_dna
                        .traits
                        .push(ParsedTrait::Float(numerator as f64 / denominator as f64));
                }
                TraitPoolUnion::MutantVec(mutant_numbers) => {
                    if mutant_numbers.is_empty() {
                        return Err(Error::DecodeEmptyMutantVecTrait);
                    }
                    let offset = (offset + rng.next_u64()) as usize % mutant_numbers.len();
                    let value = mutant_numbers.get_unchecked(offset);
                    parsed_dna.traits.push(ParsedTrait::Number(value.into()));
                }
                TraitPoolUnion::NumberRange(number_range) => {
                    let upperbound: u64 = number_range.nth1().into();
                    let lowerbound: u64 = number_range.nth0().into();
                    if upperbound < lowerbound {
                        return Err(Error::DecodeInvalidNumberRangeTrait);
                    }
                    let offset = offset % (upperbound - lowerbound);
                    parsed_dna
                        .traits
                        .push(ParsedTrait::Number(lowerbound + offset));
                }
                TraitPoolUnion::FloatRange(float_range) => {
                    let upperbound: u64 = float_range.numerator_range().nth1().into();
                    let lowerbound: u64 = float_range.numerator_range().nth0().into();
                    if upperbound < lowerbound {
                        return Err(Error::DecodeInvalidFloatRangeTrait);
                    }
                    let offset = offset % (upperbound - lowerbound);
                    let numerator = lowerbound + offset;
                    let denominator: u64 = float_range.denominator().into();
                    parsed_dna
                        .traits
                        .push(ParsedTrait::Float(numerator as f64 / denominator as f64));
                }
                TraitPoolUnion::MutantRange(mutant_number_range) => {
                    let upperbound: u64 = mutant_number_range.nth1().into();
                    let lowerbound: u64 = mutant_number_range.nth0().into();
                    if upperbound < lowerbound {
                        return Err(Error::DecodeInvalidMutantRangeTrait);
                    }
                    let offset = (offset + rng.next_u64()) % (upperbound - lowerbound);
                    parsed_dna
                        .traits
                        .push(ParsedTrait::Number(lowerbound + offset));
                }
            }
        }
        result.push(parsed_dna);
    }

    Ok(serde_json::to_string(&result).unwrap().into_bytes())
}
