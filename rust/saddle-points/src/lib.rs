pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    for row in 0..input.len() {
        let max = find_max_in_row(&input[row]);
        for (idx, val) in input[row].iter().enumerate() {
            if *val == max {
                if is_min_in_column(idx, *val, input) {
                    saddle_points.push((row, idx));
                }
            }
        }
    }

    saddle_points
}

fn is_min_in_column(idx: usize, num: u64, matrix: &[Vec<u64>]) -> bool {
    for row in matrix {
        if row[idx] < num {
            return false
        }
    }

    true
}

fn find_max_in_row(row: &[u64]) -> u64 {
    let mut max = row[0];

    for num in row.iter().skip(1) {
        if num > &max {
            max = *num;
        } 
    }

    max
}