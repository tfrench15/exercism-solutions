pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    if n % 3 == 0 {
        s.push_str("Pling");
    };
    if n % 5 == 0 {
        s.push_str("Plang");
    };
    if n % 7 == 0 {
        s.push_str("Plong");
    };

    if s != "" {
        return s;
    } else {
        return n.to_string();
    };
}
