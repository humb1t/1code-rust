use parity_scale_codec;
use onecode::ser as one_code_ser;

#[derive(serde::Serialize, Debug, PartialEq, parity_scale_codec::Encode)]
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

impl StructToSerialize {
    pub fn new() -> Self {
        StructToSerialize {
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
        }
    }
}

#[test]
fn size_1code(){
    let struct_to_serialize = StructToSerialize::new();
    let encoded = one_code_ser::to_string(&struct_to_serialize).unwrap();
    println!("1code size: {} bytes", encoded.as_bytes().len());
}

#[test]
fn size_scale() {
    let struct_to_serialize = StructToSerialize::new();
    let encoded = parity_scale_codec::Encode::encode(&struct_to_serialize);
    println!("scale size: {} bytes", encoded.len());
}

#[test]
fn size_bincode() {
    let struct_to_serialize = StructToSerialize::new();
    let encoded = bincode::serialize(&struct_to_serialize).unwrap();
    println!("bincode size: {} bytes", encoded.len());
}