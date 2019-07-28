pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    for row in input {
        let maxes = find_max_in_row(row);
        for pair in &maxes {
            if is_min_in_column(pair.0 as usize, pair.1, input) {
                saddle_points.push(*pair);
            }
        }
    }

    saddle_points
}

fn is_min_in_column(idx: usize, num: usize, matrix: &[Vec<u64>]) -> bool {
    for row in matrix {
        if row[idx] < num as u64 {
            return false
        }
    }

    true
}

fn find_max_in_row(row: &[u64]) -> Vec<(usize, usize)> {
    let mut maxes: Vec<(usize, usize)> = Vec::new();
    let mut max = row[0];
    let mut idx = 0;

    maxes.push((idx, max as usize));

    for (i, val) in row.iter().enumerate().skip(1) {
        if val > &max {
            maxes.clear();
            max = *val;
            idx = i;
            maxes.push((idx, max as usize));
        } else if *val == max {
            maxes.push((i, *val as usize));
        } else {
            continue
        }
    }

    maxes
}