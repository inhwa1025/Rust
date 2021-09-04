fn main() {
    // let length1 = 50;
    // let width1 = 30;
    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}