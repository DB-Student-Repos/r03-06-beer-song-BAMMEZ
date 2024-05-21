pub fn sing_song() -> String {
    let mut song = String::new();
    for i in (0..=99).rev() {
        song.push_str(&verse(i));
        if i > 0 {
            song.push_str("\n");
        }
    }
    song
}

pub fn verse(n: usize) -> String { // Updated function
    match n {
        0 => format!(
            "No more bottles of beer on the wall, no more bottles of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.\n"
        ),
        1 => format!(
            "1 bottle of beer on the wall, 1 bottle of beer.\n\
            Take it down and pass it around, no more bottles of beer on the wall.\n"
        ),
        _ => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\n\
            Take one down and pass it around, {1} {2} of beer on the wall.\n",
            n,
            n - 1,
            if n - 1 == 1 { "bottle" } else { "bottles" }
        ),
    }
}

pub fn sing(start: usize, end: usize) -> String { // Updated function
    let mut song = String::new();
    for i in (end..=start).rev() {
        song.push_str(&verse(i));
        if i > end {
            song.push_str("\n");
        }
    }
    song
}

