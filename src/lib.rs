pub struct CubeArray {
    pub datacube: Vec<Vec<Vec<u64>>>,
}

impl CubeArray {
    pub fn new(x:usize, y:usize, z:usize) -> CubeArray {
        CubeArray {
            datacube: vec![vec![vec![0;x]; y]; z]
        }
    }
}
