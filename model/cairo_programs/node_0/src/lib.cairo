mod initializer_0;
mod initializer_1;

use orion::operators::tensor::Tensor;
use orion::numbers::FP16x16;
use orion::operators::nn::{NNTrait, FP16x16NN};


fn main(input: Tensor<FP16x16>) -> Tensor<FP16x16> {
    let node_0 = NNTrait::gemm(
        input,
        initializer_0::get_initializer_0(),
        Option::Some(initializer_1::get_initializer_1()),
        Option::Some(FP16x16 { mag: 65536, sign: false }),
        Option::Some(FP16x16 { mag: 65536, sign: false }),
        false,
        true
    );

    node_0
}
