use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::fs::File;
use hello_cargo::Tweet;
use hello_cargo::Summarizable;

// generic type
/*fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}*/

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

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

    /*//generic type -function
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("The largest char is {}", result);*/

    //generic type -struct
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 10.4 };

    //generic type -method
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = integer_and_float.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    //trait (lib.rs 참고)
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summary());
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