mod chunk_0;
mod chunk_1;

use orion::operators::tensor::Tensor;
use orion::numbers::FP16x16;


fn get_initializer_0() -> Tensor<FP16x16> {
    let mut shape = array![10, 196];

    let mut data = array![];
    chunk_0::compute(ref data);
    chunk_1::compute(ref data);

    Tensor { shape: shape.span(), data: data.span() }
}
