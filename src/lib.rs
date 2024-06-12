#![cfg_attr(not(test), no_std)]

extern crate alloc;
pub mod decoder;

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::decoder::{dobs_decode, dobs_parse_parameters};

    #[test]
    fn test_generate_basic_example() {
        let traits_base = json!([
            [
                "Name",
                "String",
                0,
                1,
                "options",
                ["Alice", "Bob", "Charlie", "David", "Ethan", "Florence", "Grace", "Helen",],
            ],
            ["Age", "Number", 1, 1, "range", [0, 100]],
            ["Score", "Number", 2, 1, "rawNumber"],
            ["DNA", "String", 3, 3, "rawString"],
            ["URL", "String", 6, 30, "utf8"],
            ["Value", "Timestamp", 3, 3, "rawNumber"],
        ])
        .to_string();
        println!("traits_base = {traits_base}");

        let spore_dna = "ac7b88aabbcc687474703a2f2f3132372e302e302e313a383039300000";
        let parameters = dobs_parse_parameters(vec![spore_dna.as_bytes(), traits_base.as_bytes()])
            .expect("parse parameters");

        let dna_traits = dobs_decode(parameters)
            .map_err(|error| format!("error code = {}", error as u64))
            .unwrap();

        println!("dna_traits = {}\n", String::from_utf8_lossy(&dna_traits));
    }
}
