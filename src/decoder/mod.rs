use core::cmp;

use alloc::{string::String, vec::Vec};

pub mod types;
use serde_json::Value;
use types::{Error, Parameters, ParsedDNA, ParsedTrait, Pattern};

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
        let traits_pool: Value =
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
        let byte_offset = cmp::min(schema_base.offset as usize, spore_dna.len());
        let byte_end = cmp::min(byte_offset + schema_base.len as usize, spore_dna.len());
        let mut dna_segment = spore_dna[byte_offset..byte_end].to_vec();
        let value: Value = match schema_base.pattern {
            Pattern::RawNumber => Value::Number(parse_u64(dna_segment)?.into()),
            Pattern::RawString => Value::String(hex::encode(&dna_segment)),
            Pattern::Utf8 => {
                while dna_segment.last() == Some(&0) {
                    dna_segment.pop();
                }
                Value::String(
                    String::from_utf8(dna_segment).map_err(|_| Error::DecodeBadUTF8Format)?,
                )
            }
            Pattern::Range => {
                let args = schema_base.args.ok_or(Error::DecodeMissingRangeArgs)?;
                if args.len() != 2 {
                    return Err(Error::DecodeInvalidRangeArgs);
                }
                let lower = args[0].as_u64().ok_or(Error::DecodeInvalidRangeArgs)?;
                let upper = args[1].as_u64().ok_or(Error::DecodeInvalidRangeArgs)?;
                if upper <= lower {
                    return Err(Error::DecodeInvalidRangeArgs);
                }
                let offset = parse_u64(dna_segment)?;
                let offset = offset % (upper - lower);
                Value::Number((lower + offset).into())
            }
            Pattern::Options => {
                let args = schema_base.args.ok_or(Error::DecodeMissingOptionArgs)?;
                if args.is_empty() {
                    return Err(Error::DecodeInvalidOptionArgs);
                }
                let offset = parse_u64(dna_segment)?;
                let offset = offset as usize % args.len();
                args[offset].clone()
            }
        };
        parsed_dna.traits.push(ParsedTrait {
            type_: schema_base.type_,
            value,
        });
        result.push(parsed_dna);
    }

    Ok(serde_json::to_string(&result).unwrap().into_bytes())
}

fn parse_u64(dna_segment: Vec<u8>) -> Result<u64, Error> {
    let offset = match dna_segment.len() {
        1 => dna_segment[0] as u64,
        2 => u16::from_le_bytes(dna_segment.clone().try_into().unwrap()) as u64,
        3 | 4 => {
            let mut buf = [0u8; 4];
            buf[..dna_segment.len()].copy_from_slice(&dna_segment);
            u32::from_le_bytes(buf) as u64
        }
        5..=8 => {
            let mut buf = [0u8; 8];
            buf[..dna_segment.len()].copy_from_slice(&dna_segment);
            u64::from_le_bytes(buf)
        }
        _ => return Err(Error::DecodeUnexpectedDNASegment),
    };
    Ok(offset)
}
