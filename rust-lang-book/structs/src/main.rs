#[derive(Debug)]
struct Rectangle {
    l: u32,
    w: u32
}

fn main() -> Result<(), std::io::Error> {
    let rect = Rectangle{ l: 50, w: 50 };

    println!("Rect: {:?}", rect);

    println!(
        "Area: {}",
        area(&rect)
    );

    Ok(())
}

fn area(rect: &Rectangle) -> u32 {
    rect.l * rect.w
}
