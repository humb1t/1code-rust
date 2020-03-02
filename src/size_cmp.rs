use crate::test_struct::*;
use parity_scale_codec::Encode;
use crate::ser as one_code_ser;

#[test]
fn size_1code(){
    let struct_to_serialize = get_test_struct();
    let encoded = one_code_ser::to_string(&struct_to_serialize).unwrap();
    println!("1code size: {} bytes", encoded.as_bytes().len());
}

#[test]
fn size_scale() {
    let struct_to_serialize = get_test_struct();
    let encoded = struct_to_serialize.encode();
    println!("scale size: {} bytes", encoded.len());
}