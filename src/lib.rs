#![cfg_attr(not(test), no_std)]

extern crate alloc;
pub mod decoder;
pub mod schema;

#[cfg(test)]
mod test {
    use ckb_hash::{Blake2bBuilder, CKB_HASH_PERSONALIZATION};
    use ckb_types::h256;
    use molecule::prelude::{Builder, Entity};

    use crate::decoder::{dobs_decode, types::Parameters};
    use crate::schema::dob_0::{
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
        let horn_vec = trait_schema!(
            1,
            StringVec,
            vec!["Blue", "Red", "Colorful", "Gold", "White"]
        );
        let wings_vec = trait_schema!(
            1,
            StringVec,
            vec!["Blue", "Red", "Colorful", "Gold", "White"]
        );
        let spirit_body_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "Blue Wood",
                "Red Fire",
                "Colorful Earth",
                "Gold Metal",
                "White Water"
            ]
        );
        let tail_vec = trait_schema!(
            1,
            StringVec,
            vec!["Blue", "Red", "Colorful", "Gold", "White"]
        );
        let hair_vec = trait_schema!(1, StringVec, vec!["Yang-Short", "Yin-Long"]);
        let horseshoes_vec = trait_schema!(
            1,
            StringVec,
            vec!["Blue", "Red", "Colorful", "Gold", "White"]
        );
        let talent_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "Revival", "Death", "Prophet", "Curse", "Crown", "Hermit", "Guard", "Attack",
                "Calling", "Forget"
            ]
        );
        let hp_range = trait_schema!(4, NumberRange, (50000, 100000));
        let lucky_range = trait_schema!(1, NumberRange, (1, 49));

        // this traits pattern should require at least 12 bytes length of DNA
        let traits_base = TraitsBase::new_builder()
            .push(trait_pool!("horn", horn_vec))
            .push(trait_pool!("wings", wings_vec))
            .push(trait_pool!("body", spirit_body_vec))
            .push(trait_pool!("tail", tail_vec))
            .push(trait_pool!("hair", hair_vec))
            .push(trait_pool!("horseshoes", horseshoes_vec))
            .push(trait_pool!("talent", talent_vec))
            .push(trait_pool!("hp", hp_range))
            .push(trait_pool!("lucky", lucky_range))
            .build();

        let block_number = 12559090u64;
        let cell_id = {
            let dob_tx_hash =
                h256!("0xe0cc0c77de31483b27384753ec36a1f413bbbf79535c7605a882d490357de97b");
            let dob_out_index = 0u32;
            let mut hash = Blake2bBuilder::new(8)
                .personal(CKB_HASH_PERSONALIZATION)
                .build();
            hash.update(dob_tx_hash.as_bytes());
            hash.update(&dob_out_index.to_le_bytes());
            let mut cell_id = [0u8; 8];
            hash.finalize(&mut cell_id);
            u64::from_le_bytes(cell_id)
        };
        let spore_dna = {
            let mut hash = Blake2bBuilder::new(12)
                .personal(CKB_HASH_PERSONALIZATION)
                .build();
            hash.update(&block_number.to_le_bytes());
            hash.update(&cell_id.to_le_bytes());
            let mut spore_dna = [0u8; 12];
            hash.finalize(&mut spore_dna);
            spore_dna.to_vec()
        };

        println!("hexed_spore_dna = {}\n", hex::encode(&spore_dna));
        println!("block_number = {block_number}\n");
        println!("cell_id = {cell_id}\n");
        println!(
            "hexed_trats_base = {}\n",
            hex::encode(traits_base.as_slice())
        );

        let dna_traits = dobs_decode(Parameters {
            spore_dna,
            traits_base,
        })
        .map_err(|error| format!("error code = {}", error as u64))
        .unwrap();

        println!("{}", String::from_utf8_lossy(&dna_traits));
    }
}
