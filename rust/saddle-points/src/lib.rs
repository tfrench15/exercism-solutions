pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

}

fn is_min_in_column(num: u64, idx: usize, matrix: &[Vec<u64>]) -> bool {
    for row in matrix {
        if row[idx] < num {
            return false
        }
    }

    true
}

fn max_in_row(matrix: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let possible_saddle_points: Vec<(usize, usize)> = Vec::new();

    for row in matrix {
        let mut idx = 0;
        let mut max = row[0];

        for i, val in row.enumerate().skip(1) {
            if val > max {
                idx = i;
                max = val;
            }

            
        }
    }
}