use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    println!("Hello, world!");

    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //push_str
    let mut ss = String::from("foo");
    ss.push_str("bar");

    //format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    //string slicing
    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }
    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }

    //Hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}

//에러처리 - result
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4); //같음을 나타내는 매크로
    }
}