use std::cmp::PartialOrd;

fn largest_(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}


fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //if x.len() > y.len() {
        //x
    //} else {
        //y
    //}
//}
//
//fn longest<'a>(x: &str, y: &str) -> &'a str {
    //let result = String::from("really long string");
    //result.as_str()
//}


//struct Point<T> {
    //x: T,
    //y: T,
//}

//impl<T> Point<T> {
    //fn x(&self) -> &T {
        //&self.x
    //}
//}


struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

//enum Result<T, E> {
    //Ok(T),
    //Err(E),
//}
//
//

pub trait Summarizable {
   fn author_summary(&self) -> String;

   fn summary(&self) -> String {
       format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

pub trait Tisplay {}
pub trait Debug{}

fn some_function<T, U>(t: T, u: U) -> ()
    where T: Tisplay + Clone,
          U: Clone + Debug {  }


struct ImportantExcerpt<'a> {
    part: &'a str,
}

//impl<'a> ImportantExcerpt<'a> {
    //fn level(&self) -> i32
        //3
    //}
//}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {

    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summary());

    println!("1 new tweet: {}", tweet.summary());

    let string1 = String::from("long string is long");
    //let result;
    //{
        //let string2 = String::from("xyz");
        //result = longest(string1.as_str(), string2.as_str());
    //}
    println!("The longest string is {}", result);

    let novel = String::from("Call me Inshmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt{ part: first_sentence };

    let s: &'static str = "I have a static lifetime.";
}
