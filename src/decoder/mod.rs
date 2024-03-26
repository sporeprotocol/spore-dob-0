use alloc::vec::Vec;
use molecule::prelude::Entity;

use crate::schema::dob_721::{TraitPoolUnion, TraitsBase};

pub mod types;
use types::{Error, Parameters, ParsedDNA, ParsedTrait};

// example:
// argv[1] = efc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d (hexed DNA string in Spore)
// argv[2] = 1250945 (block number while minting)
// argv[3] = d48869363ff41a103b131a29f43...d7be6eeaf513c2c3ae056b9b8c2e1 (traits config in Cluster)
pub fn dobs_parse_parameters(args: Vec<&[u8]>) -> Result<Parameters, Error> {
    if args.len() != 2 {
        return Err(Error::ParseInvalidArgCount);
    }

    let spore_dna = {
        let value = args[0];
        if value.is_empty() || value.len() % 2 != 0 {
            return Err(Error::ParseInvalidSporeDNA);
        }
        hex::decode(&value).map_err(|_| Error::ParseInvalidSporeDNA)?
    };
    let traits_base = {
        let value = args[2];
        if value.len() % 2 != 0 {
            return Err(Error::ParseInvalidSporeDNA);
        }
        let traits = hex::decode(&value).map_err(|_| Error::ParseInvalidTraitsBase)?;
        TraitsBase::from_compatible_slice(&traits).map_err(|_| Error::ParseInvalidTraitsBase)?
    };
    Ok(Parameters {
        spore_dna,
        traits_base,
    })
}

pub fn dobs_decode(parameters: Parameters) -> Result<Vec<u8>, Error> {
    let Parameters {
        mut spore_dna,
        traits_base,
    } = parameters;

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
            }
        }
        result.push(parsed_dna);
    }

    Ok(serde_json::to_string(&result).unwrap().into_bytes())
}
