pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let idx = array.binary_search(&key);

    match idx {
        Err(_v) => { None },
        Ok(v) => { Some(v) }
    }
}

pub fn second_find(array: &[i32], key: i32) -> Option<usize> {
    let idx = array.binary_search(&key);
    if idx.is_err() {
        return None
    }

    idx.ok()
}

pub fn third_find(array: &[i32], key: i32) -> Option<usize> {
    match array.binary_search(&key).is_err() {
        true => { None },
        false => { array.binary_search(&key).ok() }
    }
}

