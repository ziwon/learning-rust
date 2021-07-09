use std::io;
use std::io::Read;
use std::io::ErrorKind;

use std::fs::File;

fn main() {
    //panic!("crash and burn");] let v = vec![1, 2, 3];

    //v[99];
    //
    let f = File::open("hello.txt");
    //let f = match f {
        //Ok(file) => file,
        //Err(error) => {
            //panic!("There was a problem opening the file: {:?}", error)
        //},
    //};

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!  {
                        "Tried to create file but there was a problem: {:?}", e
                    }
                }
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
    println!("{:?}", f);

//    let f2 = File::open("hello2.txt").unwrap();
//    println!("{:?}", f2);

    read_username_from_file_3();
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello3.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello4.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello5.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
