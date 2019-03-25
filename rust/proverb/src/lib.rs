pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    if list.len() == 0 {
        return proverb
    }

    let mut cur = 0;
    let mut next = 1;

    while next <= list.len() - 1 {
        let line = push_string(list[cur].to_string(), list[next].to_string());
        proverb.push_str(&line);
        cur += 1;
        next += 1;
    }

    let last = push_final(list[0].to_string());
    proverb.push_str(&last);

    proverb
}

fn push_string(current: String, next: String) -> String {
    format!("For want of a {} the {} was lost.\n", current, next)
}

fn push_final(first: String) -> String {
    format!("And all for the want of a {}.", first)
}