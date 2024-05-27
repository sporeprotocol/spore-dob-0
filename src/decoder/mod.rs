use alloc::{string::String, vec::Vec};

pub mod types;
use serde_json::Value;
use types::{ArgsType, Error, Parameters, ParsedDNA, ParsedTrait, Pattern};

use self::types::decode_trait_schema;

// example:
// argv[0] = efc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d (hexed DNA string in Spore)
// argv[1] = d48869363ff41a103b131a29f43...d7be6eeaf513c2c3ae056b9b8c2e1 (hexed pattern string in Cluster)
pub fn dobs_parse_parameters(args: Vec<&[u8]>) -> Result<Parameters, Error> {
    if args.len() != 2 {
        return Err(Error::ParseInvalidArgCount);
    }

    let spore_dna = {
        let value = args[0];
        if value.is_empty() || value.len() % 2 != 0 {
            return Err(Error::ParseInvalidSporeDNA);
        }
        hex::decode(value).map_err(|_| Error::ParseInvalidSporeDNA)?
    };
    let traits_base = {
        let value = args[1];
        let traits_pool: Vec<Vec<Value>> =
            serde_json::from_slice(value).map_err(|_| Error::ParseInvalidTraitsBase)?;
        decode_trait_schema(traits_pool)?
    };
    Ok(Parameters {
        spore_dna,
        traits_base,
    })
}

pub fn dobs_decode(parameters: Parameters) -> Result<Vec<u8>, Error> {
    let Parameters {
        spore_dna,
        traits_base,
    } = parameters;

    let mut result = Vec::new();
    for schema_base in traits_base.into_iter() {
        let mut parsed_dna = ParsedDNA {
            name: schema_base.name,
            ..Default::default()
        };
        let byte_offset = schema_base.offset as usize;
        let byte_length = schema_base.len as usize;
        if spore_dna.len() < byte_offset + byte_length {
            return Err(Error::DecodeInsufficientSporeDNA);
        }
        let mut dna_segment = spore_dna[byte_offset..byte_offset + byte_length].to_vec();
        let offset = match dna_segment.len() {
            1 => Some(dna_segment[0] as u64),
            2 => Some(u16::from_le_bytes(dna_segment.clone().try_into().unwrap()) as u64),
            4 => Some(u32::from_le_bytes(dna_segment.clone().try_into().unwrap()) as u64),
            8 => Some(u64::from_le_bytes(dna_segment.clone().try_into().unwrap())),
            _ => None,
        };
        match schema_base.pattern {
            Pattern::Raw => match schema_base.type_ {
                ArgsType::Number => {
                    let value = offset.ok_or(Error::DecodeUnexpectedDNASegment)?;
                    parsed_dna.traits.push(ParsedTrait::Number(value));
                }
                ArgsType::String => {
                    let value = hex::encode(&dna_segment);
                    parsed_dna.traits.push(ParsedTrait::String(value));
                }
            },
            Pattern::Utf8 => {
                if schema_base.type_ != ArgsType::String {
                    return Err(Error::DecodeArgsTypeMismatch);
                }
                while dna_segment.last() == Some(&0) {
                    dna_segment.pop();
                }
                let value =
                    String::from_utf8(dna_segment).map_err(|_| Error::DecodeBadUTF8Format)?;
                parsed_dna.traits.push(ParsedTrait::String(value));
            }
            Pattern::Range => {
                let args = schema_base.args.ok_or(Error::DecodeMissingRangeArgs)?;
                if args.len() != 2 {
                    return Err(Error::DecodeInvalidRangeArgs);
                }
                if schema_base.type_ != ArgsType::Number {
                    return Err(Error::DecodeArgsTypeMismatch);
                }
                let lowerbound = args[0]
                    .parse::<u64>()
                    .map_err(|_| Error::DecodeInvalidRangeArgs)?;
                let upperbound = args[1]
                    .parse::<u64>()
                    .map_err(|_| Error::DecodeInvalidRangeArgs)?;
                if upperbound <= lowerbound {
                    return Err(Error::DecodeInvalidRangeArgs);
                }
                let offset = offset.ok_or(Error::DecodeUnexpectedDNASegment)?;
                let offset = offset % (upperbound - lowerbound);
                parsed_dna
                    .traits
                    .push(ParsedTrait::Number(lowerbound + offset));
            }
            Pattern::Options => {
                let args = schema_base.args.ok_or(Error::DecodeMissingOptionArgs)?;
                if args.is_empty() {
                    return Err(Error::DecodeInvalidOptionArgs);
                }
                let offset = offset.ok_or(Error::DecodeUnexpectedDNASegment)? as usize;
                let offset = offset % args.len();
                match schema_base.type_ {
                    ArgsType::String => {
                        let value = args[offset].clone();
                        parsed_dna.traits.push(ParsedTrait::String(value));
                    }
                    ArgsType::Number => {
                        let value = args[offset]
                            .parse::<u64>()
                            .map_err(|_| Error::DecodeInvalidOptionArgs)?;
                        parsed_dna.traits.push(ParsedTrait::Number(value));
                    }
                }
            }
        }
        result.push(parsed_dna);
    }

    Ok(serde_json::to_string(&result).unwrap().into_bytes())
}
