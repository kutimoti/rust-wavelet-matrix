use crate::wv::fid_size::FIDSize;

pub struct FIDBuilder<S: FIDSize> {
    bit: Vec<u8>,
    len: usize,
    _phantom: std::marker::PhantomData<S>,
}

impl<S: FIDSize> FIDBuilder<S> {
    pub fn new(n: usize) -> Self {
        let cnum = (n + S::CW - 1) / S::CW;
        let bnum = S::CW / S::BW;
        FIDBuilder {
            bit: vec![0; cnum * bnum],
            len: n,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn set(&mut self, pos: usize) {
        let bpos = pos / S::BW;
        let offset = pos % S::BW;
        self.bit[bpos] |= 1u8 << offset;
    }

    pub fn unset(&mut self, pos: usize) {
        let bpos = pos / S::BW;
        let offset = pos % S::BW;
        self.bit[bpos] &= !(1u8 << offset);
    }
}
