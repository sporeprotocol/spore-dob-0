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

    // #[test]
    // fn test_dna_decode() {
    //     let spirits_vec = trait_schema!(
    //         1,
    //         StringVec,
    //         vec![
    //             "Wood, Blue Body",
    //             "Fire, Red Body",
    //             "Earth, Colorful Body",
    //             "Metal, Golden Body",
    //             "Water, White Body"
    //         ]
    //     );
    //     let yin_yang_vec = trait_schema!(1, StringVec, vec!["Yang, Short Hair", "Yin, Long hair"]);
    //     let talents_vec = trait_schema!(
    //         1,
    //         StringVec,
    //         vec![
    //             "Revival", "Death", "Curse", "Prophet", "Crown", "Hermit", "Attack", "Guard",
    //             "Summon", "Forget"
    //         ]
    //     );
    //     let horn_vec = trait_schema!(
    //         1,
    //         StringVec,
    //         vec![
    //             "Shaman Horn",
    //             "Hel Horn",
    //             "Necromancer Horn",
    //             "Sibyl Horn ",
    //             "Caesar Horn",
    //             "Lao Tsu Horn",
    //             "Warrior Horn",
    //             "Praetorian Horn",
    //             "Bard Horn",
    //             "Lethe Horn"
    //         ]
    //     );
    //     let wings_vec = trait_schema!(
    //         1,
    //         StringVec,
    //         vec![
    //             "Wind Wings",
    //             "Night Shadow Wings",
    //             "Lightning Wings",
    //             "Sun Wings",
    //             "Golden Wings",
    //             "Cloud Wings",
    //             "Morning Glow Wings",
    //             "Star Wings",
    //             "Spring Wings",
    //             "Moon Wings"
    //         ]
    //     );
    //     let tails_vec = trait_schema!(
    //         1,
    //         StringVec,
    //         vec![
    //             "Meteor Tails",
    //             "Rainbow Tails",
    //             "Willow Tails",
    //             "Phoenix Tails",
    //             "Sunset Shadow Tails",
    //             "Socrates Tails",
    //             "Dumbledore Tails",
    //             "Venus Tails",
    //             "Gaia Tails"
    //         ]
    //     );
    //     let horseshoes_vec = trait_schema!(
    //         1,
    //         StringVec,
    //         vec![
    //             "Ice Horseshoes",
    //             "Dimond Horseshoes",
    //             "Rock Horseshoes",
    //             "Flame Horseshoes",
    //             "Thunder Horseshoes",
    //             "Lotus Horseshoes",
    //             "Silver Horseshoes",
    //             "Golden Horseshoes",
    //             "Red Maple Horseshoes",
    //             "Blue Lake Horseshoes",
    //             "Colorful Stone Horseshoes"
    //         ]
    //     );
    //     let destiny_number_range = trait_schema!(4, NumberRange, (50000, 100000));
    //     let lucky_number_range = trait_schema!(1, NumberRange, (1, 49));

    //     // this traits pattern should require at least 12 bytes length of DNA
    //     let traits_base = TraitsBase::new_builder()
    //         .push(trait_pool!("Spirits", spirits_vec))
    //         .push(trait_pool!("Yin Yang", yin_yang_vec))
    //         .push(trait_pool!("Talents", talents_vec))
    //         .push(trait_pool!("Horn", horn_vec))
    //         .push(trait_pool!("Wings", wings_vec))
    //         .push(trait_pool!("Tails", tails_vec))
    //         .push(trait_pool!("Horseshoes", horseshoes_vec))
    //         .push(trait_pool!("Destiny Number", destiny_number_range))
    //         .push(trait_pool!("Lucky Number", lucky_number_range))
    //         .build();

    //     let block_number = 12559090u64;
    //     let cell_id = {
    //         let dob_tx_hash =
    //             h256!("0xe0cc0c77de31483b27384753ec36a1f413bbbf79535c7605a882d490357de97b");
    //         let dob_out_index = 0u32;
    //         let mut hash = Blake2bBuilder::new(8)
    //             .personal(CKB_HASH_PERSONALIZATION)
    //             .build();
    //         hash.update(dob_tx_hash.as_bytes());
    //         hash.update(&dob_out_index.to_le_bytes());
    //         let mut cell_id = [0u8; 8];
    //         hash.finalize(&mut cell_id);
    //         u64::from_le_bytes(cell_id)
    //     };
    //     let unicorn_dna = {
    //         let mut hash = Blake2bBuilder::new(12)
    //             .personal(CKB_HASH_PERSONALIZATION)
    //             .build();
    //         hash.update(&block_number.to_le_bytes());
    //         hash.update(&cell_id.to_le_bytes());
    //         let mut dna = [0u8; 12];
    //         hash.finalize(&mut dna);
    //         dna.to_vec()
    //     };

    //     println!("hexed_unicorn_dna = {}\n", hex::encode(&unicorn_dna));
    //     println!("block_number = {block_number}\n");
    //     println!("cell_id = {cell_id}\n");
    //     println!(
    //         "hexed_trats_base = {}\n",
    //         hex::encode(traits_base.as_slice())
    //     );

    //     let dna_traits = dobs_decode(Parameters {
    //         spore_dna: unicorn_dna,
    //         traits_base,
    //     })
    //     .map_err(|error| format!("error code = {}", error as u64))
    //     .unwrap();

    //     println!("{}", String::from_utf8_lossy(&dna_traits));
    // }

    #[test]
    fn test_dna_decode_unicorn() {
        // 阴金木水火土 + 阳金木水火土
        let wuxing_yinyang_vec = trait_schema!(
            1,
            StringVec,
            vec!["0<_>", "1<_>", "2<_>", "3<_>", "4<_>", "5<_>", "6<_>", "7<_>", "8<_>", "9<_>",]
        );
        // 黄蓝紫红黑 x 2 (五行决定背景颜色, 需要取余)
        let prev_bgcolor_vec = trait_schema!(
            1,
            StringVec,
            vec!["(%wuxing_yinyang):['#FFFF00', '#0000FF', '#FF00FF', '#FF0000', '#FF0000#FFFF00@\']"]
        );
        // 黑黑黑黑黑 + 白白白白白 (阴阳决定字体颜色)
        let prev_vec = trait_schema!(
            1,
            StringVec,
            vec!["(%wuxing_yinyang):['#FFFFFF', '#FFFFFF', '#FFFFFF', '#FFFFFF', '#FFFFFF', '#000000', '#000000', '#000000', '#000000', '#000000'])"]
        );
        // 与 "金木水火土" 一一对应 (需要取余)
        let spirits_vec = trait_schema!(
            1,
            StringVec,
            vec!["(%wuxing_yinyang):['Metal, Golden Body', 'Wood, Blue Body', 'Water, White Body' 'Fire, Red Body', 'Earth, Colorful Body']"]
        );
        // 阴阴阴阴阴 + 阳阳阳阳阳
        let yinyang_vec = trait_schema!(
            1,
            StringVec,
            vec!["(%wuxing_yinyang):['Yin, Long hair', 'Yin, Long hair', 'Yin, Long hair', 'Yin, Long hair', 'Yin, Long hair', 'Yang, Short Hair', 'Yang, Short Hair', 'Yang, Short Hair', 'Yang, Short Hair', 'Yang, Short Hair']"]
        );
        // 阴金木水火土 + 阳金木水火土
        let talents_vec = trait_schema!(
            1,
            StringVec,
            vec!["(%wuxing_yinyang):['Guard<~>', 'Attack<~>', 'Death<~>', 'Revival<~>', 'Forget<~>', 'Summon<~>', 'Prophet<~>', 'Curse<~>', 'Hermit<~>', 'Crown<~>']"]
        );
        // 阴金木水火土 + 阳金木水火土
        let horn_vec = trait_schema!(
            1,
            StringVec,
            vec!["(%wuxing_yinyang):['Praetorian Horn', 'Warrior Horn', 'Hel Horn', 'Shaman Horn', 'Lethe Horn', 'Bard Horn', 'Sibyl Horn ', 'Necromancer Horn', 'Lao Tsu Horn', 'Caesar Horn']"]
        );
        // 随机
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
        // 随机
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
        // 随机
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
        // 随机
        let destiny_number_range = trait_schema!(4, NumberRange, (50000, 100000));
        // 随机
        let lucky_number_range = trait_schema!(1, NumberRange, (1, 49));

        // this traits pattern should require at least 16 bytes length of DNA
        let traits_base = TraitsBase::new_builder()
            .push(trait_pool!("wuxing_yinyang", wuxing_yinyang_vec))
            .push(trait_pool!("prev.bgcolor", prev_bgcolor_vec))
            .push(trait_pool!("prev<%v>", prev_vec))
            .push(trait_pool!("Spirits", spirits_vec))
            .push(trait_pool!("Yin Yang", yinyang_vec))
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
            let mut hash = Blake2bBuilder::new(16)
                .personal(CKB_HASH_PERSONALIZATION)
                .build();
            hash.update(&block_number.to_le_bytes());
            hash.update(&cell_id.to_le_bytes());
            let mut dna = [0u8; 16];
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
                "btcfs://1bc24351a0df2e686574cd1b6346a1f55f81cff0a2e52078a6e3ad0a35cfb833i0",
                "btcfs://64f562d16e2a4a29e8c4821370fff473edfa22c26ef5808adb2404e39dc013e5i0",
                "btcfs://c29fecd6d7d7eec0cb3a2b3dfdcb6aa26081db8f9851110b7c20a0f3c617299ai0",
                "btcfs://59e87ca177ef0fd457e87e9f93627660022cf519b531e1f4e3a6dda9e5e33827i0",
                "btcfs://a3589ddcf4b7a3c6da52fe6ae4ed3296f1ede139fe9127f2697ce0dcf2703b61i0",
                "btcfs://799729ff6a16dd6af57db1a8ca6146d5673a30ad9a5976dd861d348a5eec28c4i0",
                "btcfs://88dd2ab05bb8f9c72da42afc70677ac05f476e17e0f16551dc00635ae7e9546ei0",
                "btcfs://b32e3bbb73cb877c9b411529930a5b6eb3280927b282c12486ce26901b3c2291i0",
                "btcfs://a8b19ddab338db0c52f9a284b7d95ffeaa0de34e0b874177901eb92e0f9f9d8di0",
                "btcfs://ba8b1bb9d8baee4bf24a06faa25b569410f2db96b4639f8e08ccbec05c88d79bi0",
                "btcfs://aa8986f0ef667807d4b23970e64844dde3f0622542b79a5c302539de0c35b31ei0",
                "btcfs://100f7e0f0965dc54515a3831a320881315cf5ca64ad01bed2b422616b15fd314i0",
                "btcfs://b84ec0c770aa1961a3d9498ea8a67e1282532913fc1c13e3eaf5a48de2164fb9i0",
                "btcfs://a06ba2e1614a5099176e5cc4d95de76cbeb4705a8bd7e142336278ebc290fdb3i0"
            ]
        );
        let prev_bgcolor_vec = trait_schema!(
            1,
            StringVec,
            vec![
                "#FFE3EB", "#FFC2FE", "#CEBAF7", "#B7E6F9", "#ABF4D0", "#E0DFBD", "#F9F7A7",
                "#E2BE91", "#F9C662", "#F7D6B2", "#FCA863", "#F9ACAC", "#E0E1E2", "#A3A7AA"
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
