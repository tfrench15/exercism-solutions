#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    // Case 0: handle empty lists.
    if first_list.len() == 0 && second_list.len() > 0 {
        return Comparison::Sublist
    } else if first_list.len() > 0 && second_list.len() == 0 {
        return Comparison::Superlist
    }

    // Case I: if the lengths are non-zero and equal, the result is either Equal or Unequal.
    if first_list.len() == second_list.len() {
        match first_list.iter().eq(second_list.iter()) {
            true => { return Comparison::Equal },
            false => { return Comparison::Unequal },
        }
    }

    // Case II: first list is shorter than the second list.
    if first_list.len() < second_list.len() {
        for window in second_list.windows(first_list.len()) {
            match window.iter().eq(first_list.iter()) {
                true => { return Comparison::Sublist },
                false => { continue },
            }
        }
    }

    // Case III: first list is longer than the second list.
    if first_list.len() > second_list.len() {
        for window in first_list.windows(second_list.len()) {
            match window.iter().eq(second_list.iter()) {
                true => { return Comparison::Superlist },
                false => { continue },
            }
        }
    }

    Comparison::Unequal
}
