use orion::numbers::{FixedTrait, FP16x16};

fn compute(ref a: Array<FP16x16>) {
    a.append(FP16x16 { mag: 10758, sign: true });
    a.append(FP16x16 { mag: 1617, sign: true });
    a.append(FP16x16 { mag: 2318, sign: true });
    a.append(FP16x16 { mag: 12211, sign: true });
    a.append(FP16x16 { mag: 32547, sign: false });
    a.append(FP16x16 { mag: 44148, sign: false });
    a.append(FP16x16 { mag: 7147, sign: false });
    a.append(FP16x16 { mag: 32358, sign: false });
    a.append(FP16x16 { mag: 54952, sign: true });
    a.append(FP16x16 { mag: 1753, sign: true });
}
