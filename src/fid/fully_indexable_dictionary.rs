use crate::fid::fid_size::FIDSize;
use crate::fid::fid_builder::FIDBuilder;


pub struct FullyIndexableDictionary<S: FIDSize> {
    bit: Vec<u8>,
    chunk: Vec<u16>,
    blocks: Vec<Vec<u8>>,
    len: usize,
    bnum: usize,
    _phantom: std::marker::PhantomData<S>,
}

impl<S: FIDSize> FullyIndexableDictionary<S> {
    pub fn build(builder: FIDBuilder<S>) -> Self {
        let FIDBuilder { bit, len, ..  } = builder;
        let cnum = (len + S::CW - 1) / S::CW;
        let bnum = S::CW / S::BW;
        let mut chunk = vec![0u16; cnum + 1];
        let mut blocks = vec![vec![0u8; bnum]; cnum];

        for i in 0..cnum {
            for j in 0..bnum - 1 {
                blocks[i][j + 1] = blocks[i][j] + (bit[i * bnum + j].count_ones() as u8);
            }
            chunk[i + 1] = chunk[i] + (blocks[i][bnum - 1] as u16) + (bit[(i + 1) * bnum - 1].count_ones() as u16);
        }

        FullyIndexableDictionary {
            bit: bit,
            chunk: chunk,
            blocks: blocks,
            len: len,
            bnum: bnum,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn access(&self, pos: usize) -> usize {
        let bpos = pos / S::BW;
        let offset = pos % S::BW;
        ((self.bit[bpos] >> offset) & 1) as usize
    }

    /* [0, pos) */
    pub fn rank(&self, pos: usize, flag: usize) -> usize {
        let cpos = pos / S::CW;
        let bpos = (pos % S::CW) / S::BW;
        let offset = pos % S::BW;
        let masked = (self.bit[cpos * self.bnum + bpos]) & ((1 << offset) - 1);
        let res = (self.chunk[cpos] + self.blocks[cpos][bpos] as u16 + masked.count_ones() as u16) as usize;
        match flag {
            0 => pos - res,
            1 => res,
            _ => unreachable!(),
        }
    }

    /* rank(idx) = num */
    pub fn select(&self, num: usize, flag: usize) -> Option<usize> {
        if num == 0 { Some(0) }
        else if self.rank(self.len, flag) < num { None }
        else {
            let mut ok = self.len;
            let mut ng = 0;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if self.rank(mid, flag) >= num { ok = mid; }
                else { ng = mid }
            }
            Some(ok)
        }
    }
}
