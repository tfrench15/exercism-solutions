pub struct PascalsTriangle {
    layers: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { layers: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut vec: Vec<Vec<u32>> = Vec::new();
        if self.layers == 0 {
            return vec
        } 
        vec.push(vec![1]);
        if self.layers == 1 {
            return vec
        }
        vec.push(vec![1, 1]);
        if self.layers == 2 {
            return vec
        }

        let mut row = 3;
        while row <= self.layers {
            let vec_to_push = create_vec(row as usize, &vec[vec.len()-1]);
            vec.push(vec_to_push);
            row += 1;
        }
        vec
    }
}

fn create_vec(row: usize, prev: &[u32]) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::with_capacity(row);
    vec.push(1);
    let mut i = 0;
    while i < row - 2 {
        let sum = prev[i] + prev[i+1];
        vec.push(sum);
        i += 1;
    }
    vec.push(1);
    vec
}

