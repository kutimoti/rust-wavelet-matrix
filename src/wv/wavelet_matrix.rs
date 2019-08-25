use crate::fid::fid_size::{ FIDSize };
use crate::fid::fid_builder::FIDBuilder;
use crate::fid::fully_indexable_dictionary::FullyIndexableDictionary;

pub struct WaveletMatrix<S: FIDSize> {
    mat: Vec<FullyIndexableDictionary<S>>,
    spl: Vec<usize>,
    bfr: Vec<usize>,
    depth: usize,
    len: usize,
    _phantom: std::marker::PhantomData<S>
}

impl<S: FIDSize> WaveletMatrix<S> {
    pub fn new(arr: &Vec<usize>, depth: usize) -> Self {
        let mut builders = Vec::new();
        let mut idx = (0..arr.len()).collect();
        let mut spl = Vec::new();

        for d in (0..depth).rev() {
            let mut li = Vec::new();
            let mut ri = Vec::new();
            let mut builder = FIDBuilder::new(arr.len());
            for i in idx {
                let k = (arr[i] >> d) & 1;
                if k == 0 { li.push(i); }
                else {
                    ri.push(i);
                    builder.set(i);
                }
            }
            spl.push(li.len());
            builders.push(builder);
            li.append(&mut ri);
            idx = li;
        }

        let mut bfr = vec![0; arr.len()];
        bfr.push(0);
        for i in 1..arr.len() {
            bfr[i] = if arr[idx[i]] == arr[idx[i - 1]] { bfr[i - 1] }
            else { i }
        }

        let mat = builders.into_iter().map(|b| FullyIndexableDictionary::build(b)).rev().collect();

        WaveletMatrix {
            mat: mat,
            spl: spl.into_iter().rev().collect(),
            bfr: bfr,
            depth: depth,
            len: arr.len(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn dfs_pos_x(&self, pos: usize, x: usize) -> Option<usize> {
        let WaveletMatrix { mat, spl, depth, .. } = self;
        let mut p = pos;
        for d in (0..*depth).rev() {
            let k = (x >> d) & 1;
            p = mat[d].rank(p, k) + spl[d] * k;
        }
        if p == 0 { None }
        else { Some(p - 1) }
    }
    
    /* [0, pos) */
    pub fn rank_x(&self, pos: usize, x: usize) -> usize {
        match self.dfs_pos_x(pos, x) {
            None => 0,
            Some(p) => p - self.bfr[p] + 1,
        }
    }

}

#[cfg(test)]
mod wavelet_matrix_test {
    use crate::wv::wavelet_matrix::WaveletMatrix;
    use crate::fid::fid_size::FID256_8;
    #[test]
    fn rank_x_test() {
        let vec = vec![0, 7, 2, 1, 4, 3, 6, 7, 2, 5, 0, 4, 7, 2, 6, 3];
        let wv = WaveletMatrix::<FID256_8>::new(&vec, 4);
        assert_eq!(2, wv.rank_x(10, 2));
        assert_eq!(3, wv.rank_x(vec.len(), 7));
    }
}
