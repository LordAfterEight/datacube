pub struct CubeArray {
    datacube: Vec<Vec<Vec<u64>>>,
}

impl CubeArray {
    pub fn new(&self, x:usize, y:usize, z:usize) -> CubeArray {
        CubeArray {
            datacube: vec![vec![vec![0;x]; y]; z]
        }
    }
}
