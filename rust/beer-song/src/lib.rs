pub fn verse(n: i32) -> String {
    match n {
        n if n > 2 =>
    String::from(format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)),
        n if n == 2 =>
    String::from(format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1)),
    n if n == 1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
        
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut result: String = String::new();

    let count = start - end;

    for n in start..end {
        result.push_str(&verse(n));
    }

    result
}
