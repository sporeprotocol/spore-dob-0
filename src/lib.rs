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
        ($name:expr $(, $pool:expr)+) => {{
            let pool = vec![$($pool,)+];
            let schema_vec = TraitSchemaVec::new_builder().set(pool).build();
            Trait::new_builder()
                .name(String::from($name).into())
                .schema_pool(schema_vec)
                .build()
        }}
    }

    #[test]
    fn test_dna_decode_unicorn() {
        let spirits_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "Wood, Blue Body",
                "Fire, Red Body",
                "Earth, Colorful Body",
                "Metal, Golden Body",
                "Water, White Body"
            ]
        );
        let yin_yang_vec = trait_schema!(1, StringVec, vec!["Yang, Short Hair", "Yin, Long hair"]);
        let talents_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "Revival", "Death", "Curse", "Prophet", "Crown", "Hermit", "Attack", "Guard",
                "Summon", "Forget"
            ]
        );
        let horn_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "Shaman Horn",
                "Hel Horn",
                "Necromancer Horn",
                "Sibyl Horn ",
                "Caesar Horn",
                "Lao Tsu Horn",
                "Warrior Horn",
                "Praetorian Horn",
                "Bard Horn",
                "Lethe Horn"
            ]
        );
        let wings_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "Wind Wings",
                "Night Shadow Wings",
                "Lightning Wings",
                "Sun Wings",
                "Golden Wings",
                "Cloud Wings",
                "Morning Glow Wings",
                "Star Wings",
                "Spring Wings",
                "Moon Wings"
            ]
        );
        let tails_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "Meteor Tails",
                "Rainbow Tails",
                "Willow Tails",
                "Phoenix Tails",
                "Sunset Shadow Tails",
                "Socrates Tails",
                "Dumbledore Tails",
                "Venus Tails",
                "Gaia Tails"
            ]
        );
        let horseshoes_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "Ice Horseshoes",
                "Dimond Horseshoes",
                "Rock Horseshoes",
                "Flame Horseshoes",
                "Thunder Horseshoes",
                "Lotus Horseshoes",
                "Silver Horseshoes",
                "Golden Horseshoes",
                "Red Maple Horseshoes",
                "Blue Lake Horseshoes",
                "Colorful Stone Horseshoes"
            ]
        );
        let destiny_number_range = trait_schema!(4, NumberRange, (50000, 100000));
        let lucky_number_range = trait_schema!(1, NumberRange, (1, 49));

        // this traits pattern should require at least 12 bytes length of DNA
        let traits_base = TraitsBase::new_builder()
            .push(trait_pool!("Spirits", spirits_vec))
            .push(trait_pool!("Yin Yang", yin_yang_vec))
            .push(trait_pool!("Talents", talents_vec))
            .push(trait_pool!("Horn", horn_vec))
            .push(trait_pool!("Wings", wings_vec))
            .push(trait_pool!("Tails", tails_vec))
            .push(trait_pool!("Horseshoes", horseshoes_vec))
            .push(trait_pool!("Destiny Number", destiny_number_range))
            .push(trait_pool!("Lucky Number", lucky_number_range))
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
        let unicorn_dna = {
            let mut hash = Blake2bBuilder::new(12)
                .personal(CKB_HASH_PERSONALIZATION)
                .build();
            hash.update(&block_number.to_le_bytes());
            hash.update(&cell_id.to_le_bytes());
            let mut dna = [0u8; 12];
            hash.finalize(&mut dna);
            dna.to_vec()
        };

        println!("hexed_unicorn_dna = {}\n", hex::encode(&unicorn_dna));
        println!("block_number = {block_number}\n");
        println!("cell_id = {cell_id}\n");
        println!(
            "hexed_trats_base = {}\n",
            hex::encode(traits_base.as_slice())
        );

        let dna_traits = dobs_decode(Parameters {
            spore_dna: unicorn_dna,
            traits_base,
        })
        .map_err(|error| format!("error code = {}", error as u64))
        .unwrap();

        println!("{}", String::from_utf8_lossy(&dna_traits));
    }

    #[test]
    fn test_dna_decode_nervape() {
        let prev_type_vec = trait_schema!(1, StringVec, vec!["image"]);
        let prev_bg_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "btcfs://b2f4560f17679d3e3fca66209ac425c660d28a252ef72444c3325c6eb0364393i0",
                "btcfs://95525eb82f9a14c79294769492e7f9b025911d93f5fa66b819e4bd58cdcd23f1i0",
                "btcfs://eb3910b3e32a5ed9460bd0d75168c01ba1b8f00cc0faf83e4d8b67b48ea79676i0",
                "btcfs://d049a9ac3aad84355f7a551763c8282dee0e69ab05c2b77c2699bb2fb65c5c16i0",
                "btcfs://f2397b35ea2d050458bc7ec98dcd3d9c3021aa9d44b9ca1cb44673c4eb7c8efai0",
                "btcfs://fcc0d8f3957042c7b20e8efba7b16884ab57c32fefa2651b045ba506a1201608i0",
                "btcfs://b01755a141dd65ee2a0b3bd4aa64a1e3e994a3c82b42271dc5d701c48c67f6d5i0",
                "btcfs://545b94cb1ecf2175b81c601346e4a7e05149cafc6f235330c9918e35f920e109i0",
                "btcfs://5895004e95c8a4b80f05f5314d310067a703134515d82effc2ec6eba0dda3fc9i0",
                "btcfs://c42f7a462880e4e3f9b410bab583aad700e36a539aa6671b140a2176eb2f04aci0",
                "btcfs://64059f854fc9e904d0bdf43ba7040a988da8e973c0d3b563acffa9765cb6cbf3i0",
                "btcfs://9e9afbaaddbb4f2c01237bdd2405131793389d1813f5f1a02d9771310ab30c70i0",
                "btcfs://11b6303eb7d887d7ade459ac27959754cd55f9f9e50345ced8e1e8f47f4581fai0",
                "btcfs://f1dce09bbb61961b3c61efbfa263a38511cf89dbdeed206f6ecc001a52c1fb01i0"
            ]
        );
        let prev_bgcolor_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "#FFE3EB", "#FFBDFC", "#D4C0FF", "#AFE7F9", "#ABF4D0", "#E8EABE", "#FCF8AC",
                "#EABC8B", "#FFD880", "#FFE2C7", "#FFB57D", "#FFADAB", "#E0E1E2", "#A3A7AA"
            ]
        );
        let number_range = trait_schema!(1, NumberRange, (0, 255));

        let traits_base = TraitsBase::new_builder()
            .push(trait_pool!("prev.type", prev_type_vec))
            .push(trait_pool!("prev.bg", prev_bg_vec))
            .push(trait_pool!("prev.bgcolor", prev_bgcolor_vec))
            .push(trait_pool!("Background", number_range.clone()))
            .push(trait_pool!("Suit", number_range.clone()))
            .push(trait_pool!("Upper body", number_range.clone()))
            .push(trait_pool!("Lower body", number_range.clone()))
            .push(trait_pool!("Headwear", number_range.clone()))
            .push(trait_pool!("Mask", number_range.clone()))
            .push(trait_pool!("Eyewear", number_range.clone()))
            .push(trait_pool!("Mouth", number_range.clone()))
            .push(trait_pool!("Ears", number_range.clone()))
            .push(trait_pool!("Tattoo", number_range.clone()))
            .push(trait_pool!("Accessory", number_range.clone()))
            .push(trait_pool!("Handheld", number_range.clone()))
            .push(trait_pool!("Special", number_range))
            .build();

        let btc_block_number = 834293u64;
        let token_id = 1459u16;
        let btc_receiver_address = "bc1qx9ndsrwep9j6pxc3vqralpm0a9unhhlyzy7zna";
        let nervape_dna = {
            let mut hash = Blake2bBuilder::new(16)
                .personal(CKB_HASH_PERSONALIZATION)
                .build();
            hash.update(&btc_block_number.to_le_bytes());
            hash.update(&token_id.to_le_bytes());
            hash.update(btc_receiver_address.as_bytes());
            let mut dna = [0u8; 16];
            hash.finalize(&mut dna);
            dna.to_vec()
        };

        println!("hexed_nervape_dna = {}\n", hex::encode(&nervape_dna));
        println!(
            "hexed_trats_base = {}\n",
            hex::encode(traits_base.as_slice())
        );

        let dna_traits = dobs_decode(Parameters {
            spore_dna: nervape_dna,
            traits_base,
        })
        .map_err(|error| format!("error code = {}", error as u64))
        .unwrap();

        println!("{}", String::from_utf8_lossy(&dna_traits));
    }
}
