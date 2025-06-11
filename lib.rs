struct CubeArray {
    datacube: Vec<Vec<Vec<u64>>>,
}

impl CubeArray {
    pub fn new(mut self, x:u64, y:u64, z:u64) -> Vec<Vec<Vec<u64>>> {
        self.datacube = vec![vec![vec![0;64]; 64]; 64];
        return self.datacube
    }
}
