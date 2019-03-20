pub fn verse(n: i32) -> String {
    if n >= 3 {
        return format!("{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
    } else if n == 2 {
        return format!("{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1)
    } else if n == 1 {
        return format!("{} bottle of beer on the wall, {} bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n", n, n)
    } else if n == 0 {
        return format!("No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, {} bottles of beer on the wall.\n", n+99)
    };

    "".to_string()
}

pub fn sing(start: i32, end: i32) -> String {    
    let mut song = String::new();
    let mut idx = start;
    while idx >= end {
        song.push_str(&verse(idx));
        idx -= 1;
        if idx >= end {
            song = song + "\n";
        } 
    }

    song
}
