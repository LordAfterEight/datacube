struct CubeArray {
    datacube: Vec<Vec<Vec<u64>>>,
}

impl CubeArray {
    pub fn new(mut self, x:usize, y:usize, z:usize) -> Vec<Vec<Vec<u64>>> {
        self.datacube = vec![vec![vec![0;x]; y]; z];
        return self.datacube
    }
}
