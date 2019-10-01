use crate::fid::fid_size::{ FIDSize };
use crate::fid::fid_builder::FIDBuilder;
use crate::fid::fully_indexable_dictionary::FullyIndexableDictionary;
use std::ops::Range;

pub struct WaveletMatrix<S: FIDSize> {
    mat: Vec<FullyIndexableDictionary<S>>,
    spl: Vec<usize>,
    depth: usize,
    len: usize,
    _phantom: std::marker::PhantomData<S>
}

impl<S: FIDSize> WaveletMatrix<S> {
    pub fn new(arr: &Vec<usize>, depth: usize) -> Self {
        let mut builders = Vec::new();
        let mut idx: Vec<_> = (0..arr.len()).collect();
        let mut spl = Vec::new();

        for d in (0..depth).rev() {
            let mut li = Vec::new();
            let mut ri = Vec::new();
            let mut builder = FIDBuilder::new(arr.len());
            for i in 0..arr.len() {
                let k = (arr[idx[i]] >> d) & 1;
                if k == 0 { li.push(idx[i]); }
                else {
                    ri.push(idx[i]);
                    builder.set(i);
                }
            }
            spl.push(li.len());
            builders.push(builder);
            li.append(&mut ri);
            idx = li;
        }

        let mat = builders.into_iter().map(|b| FullyIndexableDictionary::build(b)).rev().collect();

        WaveletMatrix {
            mat: mat,
            spl: spl.into_iter().rev().collect(),
            depth: depth,
            len: arr.len(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize { self.len }

    pub fn dfs_pos_x(&self, pos: usize, x: usize) -> usize {
        let WaveletMatrix { mat, spl, depth, .. } = self;
        let mut p = pos;
        for d in (0..*depth).rev() {
            let k = (x >> d) & 1;
            p = mat[d].rank(p, k) + spl[d] * k;
        }
        return p;
    }

    pub fn rank_x(&self, ran: Range<usize>, x: usize) -> usize {
        self.dfs_pos_x(ran.end, x) - self.dfs_pos_x(ran.start, x)
    }

    pub fn at(&self, pos: usize) -> usize {
        let WaveletMatrix { mat, spl, depth, .. } = self;
        let mut p = pos;
        let mut x = 0;
        for d in (0..*depth).rev() {
            let k = mat[d].access(p);
            x |= k << d;
            p = mat[d].rank(p, k) + spl[d] * k;
        }
        x
    }
}

#[cfg(test)]
mod wavelet_matrix_test {
    use crate::wv::wavelet_matrix::WaveletMatrix;
    use crate::fid::fid_size::FID256_8;

    #[test]
    fn rank_x_test() {
        let vec = vec![0, 7, 1, 1, 4, 3, 6, 7, 5, 5, 0, 4, 7, 6, 6, 3];
        let wv = WaveletMatrix::<FID256_8>::new(&vec, 4);
        assert_eq!(wv.rank_x(0..vec.len(), 4), 2);
        assert_eq!(wv.rank_x(0..9, 5), 1);
        assert_eq!(wv.rank_x(3..9, 1), 1);
    }

    #[test]
    fn at_test() {
        let vec = vec![0, 7, 1, 1, 4, 3, 6, 7, 5, 5, 0, 4, 7, 6, 6, 3];
        let wv = WaveletMatrix::<FID256_8>::new(&vec, 4);
        for i in 0..vec.len() {
            assert_eq!(vec[i], wv.at(i));
        }
    }
}
