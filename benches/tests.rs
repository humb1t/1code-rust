#[macro_use]
extern crate bencher;

use bencher::Bencher;
use onecode::ser as one_code_ser;
use parity_scale_codec::Encode;
use onecode::test_struct::*;

fn serialize_1code(bench: &mut Bencher) {
    let struct_to_serialize = get_test_struct();
    bench.iter(|| {
        one_code_ser::to_string(&struct_to_serialize).unwrap();
    });
}

fn serialize_scale(bench: &mut Bencher) {
    let struct_to_serialize = get_test_struct();
    bench.iter(|| {
        struct_to_serialize.encode();
    });
}

benchmark_group!(benches, serialize_1code, serialize_scale);
benchmark_main!(benches);
