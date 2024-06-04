#![cfg_attr(not(test), no_std)]

extern crate alloc;
pub mod decoder;

#[cfg(test)]
mod test {
    use ckb_hash::{Blake2bBuilder, CKB_HASH_PERSONALIZATION};
    use ckb_types::h256;

    use crate::decoder::{
        dobs_decode, dobs_parse_parameters,
        types::{ArgsType, Parameters, Pattern, TraitSchema},
    };

    const EXPECTED_UNICORN_RENDER_RESULT: &str = "[{\"name\":\"wuxing_yinyang\",\"traits\":[{\"String\":\"3<_>\"}]},{\"name\":\"prev.bgcolor\",\"traits\":[{\"String\":\"(%wuxing_yinyang):['#DBAB00', '#09D3FF', '#A028E9', '#FF3939', '#(135deg, #FE4F4F, #66C084, #00E2E2, #E180E2, #F4EC32)']\"}]},{\"name\":\"prev<%v>\",\"traits\":[{\"String\":\"(%wuxing_yinyang):['#000000', '#000000', '#000000', '#000000', '#000000', '#FFFFFF', '#FFFFFF', '#FFFFFF', '#FFFFFF', '#FFFFFF'])\"}]},{\"name\":\"Spirits\",\"traits\":[{\"String\":\"(%wuxing_yinyang):['Metal, Golden Body', 'Wood, Blue Body', 'Water, White Body', 'Fire, Red Body', 'Earth, Colorful Body']\"}]},{\"name\":\"Yin Yang\",\"traits\":[{\"String\":\"(%wuxing_yinyang):['Yin, Long hair', 'Yin, Long hair', 'Yin, Long hair', 'Yin, Long hair', 'Yin, Long hair', 'Yang, Short Hair', 'Yang, Short Hair', 'Yang, Short Hair', 'Yang, Short Hair', 'Yang, Short Hair']\"}]},{\"name\":\"Talents\",\"traits\":[{\"String\":\"(%wuxing_yinyang):['Guard<~>', 'Death<~>', 'Forget<~>', 'Curse<~>', 'Hermit<~>', 'Attack<~>', 'Revival<~>', 'Summon<~>', 'Prophet<~>', 'Crown<~>']\"}]},{\"name\":\"Horn\",\"traits\":[{\"String\":\"(%wuxing_yinyang):['Praetorian Horn', 'Hel Horn', 'Lethe Horn', 'Necromancer Horn', 'Lao Tsu Horn', 'Warrior Horn', 'Shaman Horn', 'Bard Horn', 'Sibyl Horn', 'Caesar Horn']\"}]},{\"name\":\"Wings\",\"traits\":[{\"String\":\"Sun Wings\"}]},{\"name\":\"Tail\",\"traits\":[{\"String\":\"Meteor Tail\"}]},{\"name\":\"Horseshoes\",\"traits\":[{\"String\":\"Silver Horseshoes\"}]},{\"name\":\"Destiny Number\",\"traits\":[{\"Number\":65321}]},{\"name\":\"Lucky Number\",\"traits\":[{\"Number\":35}]}]";
    const EXPECTED_NERVAPE_RENDER_RESULT: &str = "[{\"name\":\"prev.type\",\"traits\":[{\"String\":\"image\"}]},{\"name\":\"prev.bg\",\"traits\":[{\"String\":\"btcfs://59e87ca177ef0fd457e87e9f93627660022cf519b531e1f4e3a6dda9e5e33827i0\"}]},{\"name\":\"prev.bgcolor\",\"traits\":[{\"String\":\"#CEBAF7\"}]},{\"name\":\"Background\",\"traits\":[{\"Number\":170}]},{\"name\":\"Suit\",\"traits\":[{\"Number\":236}]},{\"name\":\"Upper body\",\"traits\":[{\"Number\":53}]},{\"name\":\"Lower body\",\"traits\":[{\"Number\":189}]},{\"name\":\"Headwear\",\"traits\":[{\"Number\":175}]},{\"name\":\"Mask\",\"traits\":[{\"Number\":153}]},{\"name\":\"Eyewear\",\"traits\":[{\"Number\":126}]},{\"name\":\"Mouth\",\"traits\":[{\"Number\":14}]},{\"name\":\"Ears\",\"traits\":[{\"Number\":165}]},{\"name\":\"Tattoo\",\"traits\":[{\"Number\":231}]},{\"name\":\"Accessory\",\"traits\":[{\"Number\":78}]},{\"name\":\"Handheld\",\"traits\":[{\"Number\":240}]},{\"name\":\"Special\",\"traits\":[{\"Number\":70}]}]";

    #[test]
    fn test_generate_basic_example() {
        let name = TraitSchema::new(
            "Name",
            ArgsType::String,
            0,
            1,
            Pattern::Options,
            Some(vec![
                "Alice", "Bob", "Charlie", "David", "Ethan", "Florence", "Grace", "Helen",
            ]),
        );
        let age = TraitSchema::new(
            "Age",
            ArgsType::Number,
            1,
            1,
            Pattern::Range,
            Some(vec!["0", "100"]),
        );
        let score = TraitSchema::new("Score", ArgsType::Number, 2, 1, Pattern::Raw, None);
        let dna = TraitSchema::new("DNA", ArgsType::String, 3, 3, Pattern::Raw, None);
        let url = TraitSchema::new("URL", ArgsType::String, 6, 30, Pattern::Utf8, None);
        let value = TraitSchema::new("Value", ArgsType::Number, 3, 3, Pattern::Raw, None);

        let schemas = vec![name, age, score, dna, url, value];
        let traits_base =
            serde_json::to_string(&schemas.iter().map(|v| v.encode()).collect::<Vec<_>>())
                .expect("stringify traits_base");
        println!("trats_base = {traits_base}\n");

        let spore_dna = "ac7b88aabbcc687474703a2f2f3132372e302e302e313a383039300000";
        let parameters = dobs_parse_parameters(vec![spore_dna.as_bytes(), traits_base.as_bytes()])
            .expect("parse parameters");

        let dna_traits = dobs_decode(parameters)
            .map_err(|error| format!("error code = {}", error as u64))
            .unwrap();

        println!("dna_traits = {}\n", String::from_utf8_lossy(&dna_traits));
    }

    #[test]
    fn test_dna_decode_unicorn() {
        // 阴金木水火土 + 阳金木水火土
        let wuxing_yinyang = TraitSchema::new(
            "wuxing_yinyang",
            ArgsType::String,
            0,
            1,
            Pattern::Options,
            Some(vec![
                "0<_>", "1<_>", "2<_>", "3<_>", "4<_>", "5<_>", "6<_>", "7<_>", "8<_>", "9<_>",
            ]),
        );
        // 黄蓝紫红黑 x 2 (五行决定背景颜色, 需要取余)
        let prev_bgcolor = TraitSchema::new(
            "prev.bgcolor",
            ArgsType::String,
            1,
            1,
            Pattern::Options,
            Some(vec!["(%wuxing_yinyang):['#DBAB00', '#09D3FF', '#A028E9', '#FF3939', '#(135deg, #FE4F4F, #66C084, #00E2E2, #E180E2, #F4EC32)']"]),
        );
        // 黑黑黑黑黑 + 白白白白白 (阴阳决定字体颜色)
        let prev = TraitSchema::new(
            "prev<%v>",
            ArgsType::String,
            2,
            1,
            Pattern::Options,
            Some(vec!["(%wuxing_yinyang):['#000000', '#000000', '#000000', '#000000', '#000000', '#FFFFFF', '#FFFFFF', '#FFFFFF', '#FFFFFF', '#FFFFFF'])"]),
        );
        // 与 "金木水火土" 一一对应 (需要取余)
        let spirits = TraitSchema::new(
            "Spirits",
            ArgsType::String,
            3,
            1,
            Pattern::Options,
            Some(vec!["(%wuxing_yinyang):['Metal, Golden Body', 'Wood, Blue Body', 'Water, White Body', 'Fire, Red Body', 'Earth, Colorful Body']"]),
        );
        // 阴阴阴阴阴 + 阳阳阳阳阳
        let yinyang = TraitSchema::new(
            "Yin Yang",
            ArgsType::String,
            4,
            1,
            Pattern::Options,
            Some(vec!["(%wuxing_yinyang):['Yin, Long hair', 'Yin, Long hair', 'Yin, Long hair', 'Yin, Long hair', 'Yin, Long hair', 'Yang, Short Hair', 'Yang, Short Hair', 'Yang, Short Hair', 'Yang, Short Hair', 'Yang, Short Hair']"]),
        );
        // 阴金木水火土 + 阳金木水火土
        let talents = TraitSchema::new(
            "Talents",
            ArgsType::String,
            5,
            1,
            Pattern::Options,
            Some(vec!["(%wuxing_yinyang):['Guard<~>', 'Death<~>', 'Forget<~>', 'Curse<~>', 'Hermit<~>', 'Attack<~>', 'Revival<~>', 'Summon<~>', 'Prophet<~>', 'Crown<~>']"]),
        );
        // 阴金木水火土 + 阳金木水火土
        let horn = TraitSchema::new(
            "Horn",
            ArgsType::String,
            6,
            1,
            Pattern::Options,
            Some(vec!["(%wuxing_yinyang):['Praetorian Horn', 'Hel Horn', 'Lethe Horn', 'Necromancer Horn', 'Lao Tsu Horn', 'Warrior Horn', 'Shaman Horn', 'Bard Horn', 'Sibyl Horn', 'Caesar Horn']"]),
        );
        // 随机
        let wings = TraitSchema::new(
            "Wings",
            ArgsType::String,
            7,
            1,
            Pattern::Options,
            Some(vec![
                "Wind Wings",
                "Night Shadow Wings",
                "Lightning Wings",
                "Sun Wings",
                "Golden Wings",
                "Cloud Wings",
                "Morning Glow Wings",
                "Star Wings",
                "Spring Wings",
                "Moon Wings",
                "Angel Wings",
            ]),
        );
        // 随机
        let tail = TraitSchema::new(
            "Tail",
            ArgsType::String,
            8,
            1,
            Pattern::Options,
            Some(vec![
                "Meteor Tail",
                "Rainbow Tail",
                "Willow Tail",
                "Phoenix Tail",
                "Sunset Shadow Tail",
                "Socrates Tail",
                "Dumbledore Tail",
                "Venus Tail",
                "Gaia Tail",
            ]),
        );
        // 随机
        let horseshoes = TraitSchema::new(
            "Horseshoes",
            ArgsType::String,
            9,
            1,
            Pattern::Options,
            Some(vec![
                "Ice Horseshoes",
                "Crystal Horseshoes",
                "Maple Horseshoes",
                "Flame Horseshoes",
                "Thunder Horseshoes",
                "Lotus Horseshoes",
                "Silver Horseshoes",
            ]),
        );
        // 随机
        let destiny_number = TraitSchema::new(
            "Destiny Number",
            ArgsType::Number,
            10,
            4,
            Pattern::Range,
            Some(vec!["50000", "100000"]),
        );
        // 随机
        let lucky_number = TraitSchema::new(
            "Lucky Number",
            ArgsType::Number,
            14,
            1,
            Pattern::Range,
            Some(vec!["1", "49"]),
        );

        // this traits pattern should require at least 16 bytes length of DNA
        let traits_base = vec![
            wuxing_yinyang,
            prev_bgcolor,
            prev,
            spirits,
            yinyang,
            talents,
            horn,
            wings,
            tail,
            horseshoes,
            destiny_number,
            lucky_number,
        ];

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
            "trats_base = {}\n",
            serde_json::to_string(&traits_base.iter().map(|v| v.encode()).collect::<Vec<_>>())
                .expect("stringify traits_base")
        );

        let dna_traits = dobs_decode(Parameters {
            spore_dna: unicorn_dna,
            traits_base,
        })
        .map_err(|error| format!("error code = {}", error as u64))
        .unwrap();

        let render_result = String::from_utf8_lossy(&dna_traits).to_string();
        assert_eq!(render_result, EXPECTED_UNICORN_RENDER_RESULT);
    }

    #[test]
    fn test_dna_decode_nervape() {
        let prev_type = TraitSchema::new(
            "prev.type",
            ArgsType::String,
            0,
            1,
            Pattern::Options,
            Some(vec!["image"]),
        );
        let prev_bg = TraitSchema::new(
            "prev.bg",
            ArgsType::String,
            1,
            1,
            Pattern::Options,
            Some(vec![
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
                "btcfs://a06ba2e1614a5099176e5cc4d95de76cbeb4705a8bd7e142336278ebc290fdb3i0",
            ]),
        );
        let prev_bgcolor = TraitSchema::new(
            "prev.bgcolor",
            ArgsType::String,
            2,
            1,
            Pattern::Options,
            Some(vec![
                "#FFE3EB", "#FFC2FE", "#CEBAF7", "#B7E6F9", "#ABF4D0", "#E0DFBD", "#F9F7A7",
                "#E2BE91", "#F9C662", "#F7D6B2", "#FCA863", "#F9ACAC", "#E0E1E2", "#A3A7AA",
            ]),
        );
        let other_traits = [
            "Background",
            "Suit",
            "Upper body",
            "Lower body",
            "Headwear",
            "Mask",
            "Eyewear",
            "Mouth",
            "Ears",
            "Tattoo",
            "Accessory",
            "Handheld",
            "Special",
        ]
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            TraitSchema::new(
                name,
                ArgsType::Number,
                3 + i as u64,
                1,
                Pattern::Range,
                Some(vec!["0", "255"]),
            )
        })
        .collect();

        let traits_base = vec![vec![prev_type, prev_bg, prev_bgcolor], other_traits].concat();

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
            "trats_base = {}\n",
            serde_json::to_string(&traits_base.iter().map(|v| v.encode()).collect::<Vec<_>>())
                .expect("stringify traits_base")
        );

        let dna_traits = dobs_decode(Parameters {
            spore_dna: nervape_dna,
            traits_base,
        })
        .map_err(|error| format!("error code = {}", error as u64))
        .unwrap();

        let render_result = String::from_utf8_lossy(&dna_traits).to_string();
        assert_eq!(render_result, EXPECTED_NERVAPE_RENDER_RESULT);
    }
}
