#![cfg_attr(not(test), no_std)]

extern crate alloc;
pub mod decoder;
pub mod schema;

#[cfg(test)]
mod test {
    use molecule::prelude::{Builder, Entity};

    use crate::decoder::{dobs_decode, types::Parameters};
    use crate::schema::dob_721::{
        Trait, TraitPool, TraitPoolOpt, TraitPoolUnion, TraitSchema, TraitSchemaVec, TraitsBase,
    };

    macro_rules! trait_schema {
        ($bl:expr, $uni:ident, $val:expr) => {{
            let trait_pool_opt = TraitPoolOpt::new_builder()
                .set(Some(
                    TraitPool::new_builder()
                        .set(TraitPoolUnion::$uni($val.into()))
                        .build(),
                ))
                .build();
            TraitSchema::new_builder()
                .byte_length($bl.into())
                .trait_pool(trait_pool_opt)
                .build()
        }};
        ($bl:expr) => {{
            let trait_pool_opt = TraitPoolOpt::new_builder().set(None).build();
            TraitSchema::new_builder()
                .byte_length($bl.into())
                .trait_pool(trait_pool_opt)
                .build()
        }};
    }

    macro_rules! trait_pool {
        ($name:expr $(, $pool:ident)+) => {{
            let pool = vec![$($pool,)+];
            let schema_vec = TraitSchemaVec::new_builder().set(pool).build();
            Trait::new_builder()
                .name(String::from($name).into())
                .schema_pool(schema_vec)
                .build()
        }}
    }

    #[test]
    fn test_dna_decode() {
        let pool_string_vec = trait_schema!(1, StringVec, vec!["white", "blue", "green"]);
        let pool_number_vec = trait_schema!(1, NumberVec, vec![100, 200, 300, 500]);
        let pool_float_vec = trait_schema!(1, FloatVec, (vec![1, 2, 3, 4, 5], 10));
        let pool_mutant_vec = trait_schema!(1, MutantVec, vec![1, 2, 3, 4, 5]);
        let pool_number_range = trait_schema!(1, NumberRange, (10, 90));
        let pool_float_range = trait_schema!(1, FloatRange, ((1, 9), 10));
        let pool_mutant_range = trait_schema!(1, MutantRange, (100, 300));
        let pool_none = trait_schema!(1);

        let traits_base = TraitsBase::new_builder()
            .push(trait_pool!("color", pool_string_vec))
            .push(trait_pool!("power", pool_number_vec, pool_float_vec))
            .push(trait_pool!("speed", pool_float_range))
            .push(trait_pool!("sex", pool_mutant_vec))
            .push(trait_pool!("skill", pool_number_range, pool_mutant_range))
            .push(trait_pool!("lucky", pool_none))
            .build();
        let spore_dna = vec![2, 0, 3, 13, 24, 101, 9, 240];
        println!("hexed_trats_base = {}", hex::encode(traits_base.as_slice()));
        println!("hexed_spore_dna = {}", hex::encode(&spore_dna));

        let dna_traits = dobs_decode(Parameters {
            spore_dna,
            block_number: 1250945,
            traits_base,
        })
        .map_err(|error| format!("error code = {}", error as u64))
        .unwrap();

        println!("output = {}", String::from_utf8_lossy(&dna_traits));
    }
}
