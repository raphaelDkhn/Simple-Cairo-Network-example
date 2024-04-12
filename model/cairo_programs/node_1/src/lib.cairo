use orion::operators::tensor::Tensor;
use orion::numbers::FP16x16;
use orion::operators::nn::{NNTrait, FP16x16NN};


fn main(node_0: Tensor<FP16x16>) -> Tensor<FP16x16> {
    let node_1 = NNTrait::relu(@node_0);

    node_1
}
