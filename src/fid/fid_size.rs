pub trait FIDSize {
    const CW: usize;
    const BW: usize;
}

pub struct FID256_8;
impl FIDSize for FID256_8 {
    const CW: usize = 256;
    const BW: usize = 8;
}
