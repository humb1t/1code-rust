pub fn get_test_struct() -> StructToSerialize{
    let struct_to_serialize = StructToSerialize {
        boolean: true,
        positive_integer: 1,
        negative_integer: -1,
        empty_string: "".to_string(),
        number_string: "0.1".to_string(),
        latin_string: "hello world".to_string(),
        cyrillic_string: "привет мир".to_string(),
        japan_string: "こんにちは世界".to_string(),
        number_list: vec![1, 2, 3],
        string_list: vec!["1".to_string(), "01".to_string(), "011".to_string()],
        null: Option::None,
    };
    struct_to_serialize
}

use parity_scale_codec::Encode;

#[derive(serde::Serialize, Debug, PartialEq, Encode)]
pub struct StructToSerialize {
    boolean: bool,
    positive_integer: u8,
    negative_integer: i16,
    empty_string: String,
    number_string: String,
    latin_string: String,
    cyrillic_string: String,
    japan_string: String,
    number_list: Vec<u16>,
    string_list: Vec<String>,
    null: Option<()>,
}