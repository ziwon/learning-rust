fn main() -> Result<(), std::io::Error> {
    let d = dangle1();
//    let d = dangle2();
    println!("{} world!", d);
    Ok(())
}

fn dangle1() -> String {
    let s = String::from("hello");
    s
}

//fn dangle2() -> &String {
    //let s = String::from("hello");
    //&s
//}

