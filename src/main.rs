// #[derive(Copy, Clone, Debug)]
// struct Point(i32,i32);

// #[derive(Debug)]
// struct Point(i32,i32);

// #[derive(Debug)]
// struct Highlight<'doc>(&'doc str);

// const DIGEST_SIZE: usize = 3;
// const ZERO: Option<u8> = Some(42);

// static BANNER: &str = "Welcome to RUSTOS 3.14";

// struct Point(i32, i32);
// struct Libray {
//     books: Vec<Book>
// }

// struct Book {
//     title: String,
//     year: u16,
// }

// impl Book {
//     fn new(title: &str, year: u16) -> Book {
//         Book {
//              title: String::from(title),
//              year,
//         }
//     }
// }

// impl std::fmt::Display for Book {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} ({})", self.title, self.year)
//     }
// }

// impl Libray {
//     fn new() -> Libray {
//         Libray { books: vec![] }
//     }
//     fn len(&self) -> usize {
//         self.books.len()
//     }
//     fn is_empty(&self) -> bool {
//         if self.books.len() == 0 {true}
//         else {false}
//     }
//     fn add_book(&mut self, book:Book) {
//         self.books.push(book);
//     }
//     fn print_book(&self) {
//         for i in &self.books {
//             println!("{i}");
//         }
//     }
//     fn oldest_book<'a>(&'a self) -> Option<&'a Book> {
//         self.books.get(0)
//     }
// }

use std::vec;

fn main() {
//    println!("Hello World!");

//    let mut x: i32 = 6;

//    print!("{x}");

//    while x != 1 {
//         if x % 2 == 0 {
//             x = x / 2;
//         }
//         else {
//             x = 3 * x + 1;
//         }
//         print!(" -> {x}");
//    }
//    println!();

    // let mut a: [i8; 10] = [42; 10];

    // a[5] = 0;

    // println!("a : {:?}", a);

    // let t: (i8, bool) = (7, true);

    // println!("1st index: {}",t.0);
    // println!("2nd index: {}",t.1);

    // let mut x: i32 = 10;
    // let ref_x: &mut i32 = &mut x;

    // *ref_x = 20;

    // println!("x : {x}");


    // 이렇게 하면 안된다. dangling references를 정적으로 금지한다.
    // let ref_x: &i32;
    // {
    //     let x:i32 = 10;
    //     ref_x = &x;
    // }

    // println!("ref_x : {ref_x}");

    // let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    // println!("a: {a:?}");

    // a[3] = 234;

    // let s: &[i32] = &a[2..4];

    // println!("s: {s:?}");

    // let s1: &str = "Hello";

    // println!("s1: {s1}");

    // let mut s2: String = String::from("Hello ");
    // println!("s2: {s2}");
    // s2.push_str(s1);
    // println!("s2: {s2}");

    // fizzbuzz_to(20);
    
    // let x: i8 = 15;
    // let y: i16 = 1000;

    // println!("{x} * {y} = {}", multiply(x.into(), y.into()));

    // let array = [10, 20, 30];

    // for n in array {
    //     print!(" {n}");
    // }
    // println!();

    // print!("Iterating over range:");
    // for i in 0..3 {
    //     print!(" {}", array[i]);
    // }

    // let matrix = [
    //     [101, 102, 103],
    //     [201, 202, 106],
    //     [301, 302, 303],
    // ];

    // println!("matrix:");
    // pretty_print(&matrix);

    // let transmatrix = transpose(matrix);
    // pretty_print(&transmatrix);

    // let x: i32 = 10;
    // println!("x: {x}");

    // let x = 10;
    // let y = 20;

    // takes_u32(x);
    // takes_i8(y);

    // let digest = compute_digest("Hello");
    // println!("Digest : {digest:?}");

    // println!("{BANNER}");

    // let a = 10;
    // println!("before : {a}");

    // {
    //     let a: &str = "Hello";
    //     println!("inner scope : {a}");

    //     let a = true;
    //     println!("shadowed in inner scope: {a}");
    // }
    // println!("after: {a}");

    // let s1 = String::from("Hello");

    // {
    //     let p = Point(3,4);
    //     println!("x : {}", p.0);
    // }
    // println!("y: {}", p.1);

    // let s1: String = String::from("Hello");

    // let s2: String = s1;

    // println!("s2 : {s2}");
    // println!("s1 : {s1}");

    // let s1: String = String::from("Rust");
    // let s2: String = s1;

    // let name = String::from("Alice");
    // say_hello(name);

    // let x = 42;
    // let y = x;

    // println!("x : {x}");
    // println!("y : {y}");

    // let p1 = Point(3, 4);
    // let p2 = p1;

    // println!("p1 : {p1:?}");
    // println!("p2 : {p2:?}");

    // let p1 = Point(3, 4);
    // let p2 = Point(10, 20);
    // let p3 = add(&p1, &p2);

    // println!("{p1:?} + {p2:?} = {p3:?}");

    // let mut a: i32 = 10;

    // let b: &i32 = &a;

    // {
    //     let c: &mut i32 = &mut a;
    //     *c = 20;
    // }

    // println!("a : {a}");
    // println!("b : {b}");

    // let p1: Point = Point(10, 20);
    // let p2: Point = Point(20, 20);
    // let p3: &Point = left_most(&p1, &p2);

    // println!("left-most point {:?}", p3);

    // let text = String::from("The quick bromn fox jumps over ther lazy dog.");
    // let fox = Highlight(&text[4..19]);
    // let dog = Highlight(&text[35..43]);

    // erase(text);

    // println!("{fox:?}");
    // println!("{dog:?}");

    // let mut vec = vec![10, 20];
    // vec.push(30);

    // println!("middle value: {}", vec[vec.len() / 2]);

    // for item in vec.iter() {
    //     println!("item: {item}");
    // }

    // This shows the desired behavior. Uncomment the code below and
    // implement the missing methods. You will need to update the
    // method signatures, including the "self" parameter!
    // let mut library = Libray::new();

    // println!("Our library is empty: {}", library.is_empty());
    
    // library.add_book(Book::new("Lord of the Rings", 1954));
    // library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    
    // library.print_book();
    
    // match library.oldest_book() {
    //    Some(book) => println!("My oldest book is {book}"),
    //    None => println!("My library is empty!"),
    // }
    
    // println!("Our library has {} books", library.len());

    // let v: Vec<i8> = vec![10,20,30];
    // let mut iter = v.iter();

    // println!("v[0]: {:?}", iter.next());
    // println!("v[1]: {:?}", iter.next());
    // println!("v[2]: {:?}", iter.next());
    // println!("No more items: {:?}", iter.next());

    // let v: Vec<i8> = vec![10,20,30];
    // let mut iter = v.iter();

    // let v0 = iter.next();
    // println!("v0: {v0:?}");

    // let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    // let mut iter = v.into_iter();

    // let v0 = iter.next();
    // println!("v0: {v0:?}");

    // let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    // for word in &v {
    //     println!("word: {word}");
    // }

    // for word in v {
    //     println!("word: {word}");
    // }

    // for word in v.iter() {
    //     println!("word : {word}");
    // }

    // for word in v.into_iter() {
    //     println!("word : {word}");
    // }


}

// fn erase(text: String) {
//     println!("Bye {text}!");
// }

// fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
//     if p1.0 < p2.0 {
//         p1
//     }
//     else {
//         p2
//     }
// }

// fn add(p1: &Point, p2: &Point) -> Point {
//     Point(p1.0 + p2.0, p1.1 + p2.1 )
// }

// fn say_hello(name: String) {
//     println!("Hello {name}");
// }

// fn takes_u32(x: u32) {
//     println!("u32: {x}");
// }

// fn takes_i8(y: i8) {
//     println!("i8: {y}");
// }

// fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
//     let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
//     for (idx, &b) in text.as_bytes().iter().enumerate() {
//         digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
//     }
//     digest
// }

// fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
//     let mut temp = matrix;

//     for i in 0..3 {
//         for j in 0..3 {
//             temp[j][i] = matrix[i][j];
//         }
//     }

//     temp
// }

// fn pretty_print(matrix: &[[i32; 3]; 3]) {
//     for i in 0..3 {
//         for j in matrix[i] {
//             print!("{} ", j);
//         }
//         println!();
//     }
// }

// fn multiply(x :f32, y:f32) -> f32 {
//     x * y
// }

// fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
//     if rhs == 0 {
//         return false;
//     }
//     lhs % rhs == 0
// }

// fn fizzbuzz(n: u32) -> () {
//     match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
//         (true, true) => println!("fizzbuzz"),
//         (true, false) => println!("fizz"),
//         (false, true) => println!("buzz"),
//         (false, false) => println!("{n}"),
//     }
// }

// fn fizzbuzz_to(n: u32) {
//     for n in 1..=n {
//         fizzbuzz(n);
//     }
// }

///////////////////////////////////////////////////////////////
///////////////// DAY 2 ///////////////////////////////////////
//////////////////////////////////////////////////////////////
/// 
/// // use std::vec;

// use std::ops;

// use core::num::dec2flt::number;
// use std::{collections::HashMap, hash::Hash};

// use std::rc::Rc;

// struct Person {
//     name: String,
//     age: u8,
// }

// struct Point (i32,i32);

// struct PoundOfForce(f64);
// struct Newtons(f64);

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// impl Person {
//     fn new(name: String, age: u8) -> Person {
//         Person {name, age}
//     }
// }

// #[derive(Debug)]
// enum CoinFlip {
//     Heads,
//     Tails,
// }

// enum WebEvent {
//     PageLoad,
//     KeyPress(char),
//     Click {x: i64, y: i64},
// }

// use std::mem::{align_of, size_of};

// macro_rules! dbg_size {
//     ($t:ty) => {
//         println!("{}: size {} bytes, align: {} bytes",
//                 stringify!($t), size_of::<$t>(), align_of::<$t>());
//     };
// }

// enum Foo {
//     A,
//     B,
// }

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// impl Person {
//     fn say_hello(&self) {
//         println!("Hello, my name is {}", self.name);
//     }
// }


// #[derive(Debug)]
// struct Race {
//     name: String,
//     laps: Vec<i32>
// }

// impl Race {
//     // static method가 된단다 
//     fn new(name: &str) -> Race {
//         Race { name: String::from(name), laps: Vec::new() }
//     }

//     fn add_lap(&mut self, lap: i32) {
//         self.laps.push(lap);
//     }

//     fn print_laps(&self) {
//         println!("Recorded {} laps for {}:", self.laps.len(), self.name);
//         for (idx, lap) in self.laps.iter().enumerate() {
//             println!("Lap {idx}: {lap} sec");
//         }

//     }

//     fn finsh(self) {
//         let total = self.laps.iter().sum::<i32>();

//         println!("Race {} is finished, total lap time is {}", self.name, total);
//     }
// }

// enum Result {
//     Ok(i32),
//     Err(String),
// }

// struct Foo {
//     x: (u32, u32),
//     y: u32,
// }

// struct User {
//     name: String,
//     age: u32,
//     weight: f32,
// }

// impl User {
//     pub fn new(name:String, age:u32, weight: f32) -> Self {
//         User {
//             name,
//             age,
//             weight
//         }
//     }

//     pub fn age(&self) -> u32 {
//         self.age
//     }

//     pub fn weight(&self) -> f32 {
//         self.weight
//     }

//     pub fn set_age(&mut self, new_age:u32) {
//         self.age = new_age;
//     }

//     pub fn set_weight(&mut self, new_weight:f32) {
//         self.weight = new_weight;
//     }
// }

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct Point {
//     x:i32,
//     y:i32,
// }

// impl Point {
//     pub fn new(x:i32, y:i32) -> Point {
//         Point {x, y}
//     }

//     pub fn magnitude(&self) -> f64 {
//         let mul = (self.x * self.x) + (self.y * self.y);

//         f64::sqrt(mul.into())
//     }
// }

// impl ops::Add<Point> for Point {
//     type Output = Point;

//     fn add(self, _rhs: Point) -> Point {
//         Point{
//             x: self.x + _rhs.x,
//             y: self.y + _rhs.y,
//         }
//     }
// }

// #[derive(Debug,PartialEq)]
// pub struct Polygon{
//     point: Vec<Point>
// }

// impl Polygon{
//     pub fn new() -> Polygon {
//         Polygon { point: vec![] }
//     }

//     pub fn add_point(&mut self, _point:Point) {
//         self.point.push(_point);
//     }

//     pub fn left_most_point(self) -> Option<Point> {
//         self.point.into_iter().next()
//     }
// }



// pub struct Cricle {
//     point : Point,
//     y: i32,
// }

// impl Cricle {
//     fn new(point:Point, y:i32) -> Cricle {
//         Cricle { point, y }
//     } 
// }

// pub enum Shape{
//     Polygon(Polygon),
//     Cricle(Cricle),
// }

// impl From<Polygon> for Shape {
//     fn from(poly: Polygon) -> Shape {
//         Shape::Polygon(poly)
//     }
// }

// impl From<Cricle> for Shape {
//     fn from(circle: Cricle) -> Shape {
//         Shape::Cricle(circle)
//     }
// }

// impl Shape {
//     fn circumference(&self) -> f64 {
        
//     }

//     fn round_two_digits(x: f64) -> f64 {
//         (x * 100.0).round() / 100.0
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn round_two_digits(x: f64) -> f64 {
//         (x * 100.0).round() / 100.0
//     }

//     #[test]
//     fn test_point_magnitude() {
//         let p1 = Point::new(12,13);
//         assert_eq!(round_two_digits(p1.magnitude()), 17.69);
//     }

//     #[test]
//     fn test_point_add() {
//         let p1 = Point::new(16,16);
//         let p2 = p1 + Point::new(-4, 3);
//         assert_eq!(p2, Point::new(12, 19));
//     }

//     #[test]
//     fn test_polygon_left_most_point() {
//         let p1 = Point::new(12, 13);
//         let p2 = Point::new(16, 16);

//         let mut poly = Polygon::new();
//         poly.add_point(p1);
//         poly.add_point(p2);
//         assert_eq!(poly.left_most_point(), Some(p1));
//     }

//     #[test]
//     fn test_shape_circumferences() {
//         let mut poly = Polygon::new();
//         poly.add_point(Point::new(12, 13));
//         poly.add_point(Point::new(17, 11));
//         poly.add_point(Point::new(12, 13));

//         let shapes = vec![
//             Shape::from(poly),
//             Shape::from(Cricle::new(Point::new(10, 20), 5)),
//         ];

//         let circumferences = shapes
//         .iter()
//         .map(Shape::circumference)
//         .map(round_two_digits)
//         .collect::<Vec<_>>();

//         assert_eq!(circumferences, vec![15.48, 31.42]);
//     }

// }

// use std::vec;

// #[derive(Debug)]
// enum List<T> {
//     Cons(T, Box<List<T>>),
//     Nil,
// }

// mod foo {
//     pub fn do_something() {
//         println!("In the foo modules");
//     }
// }

// mod bar {
//     pub fn do_something() {
//         println!("In the bar module");
//     }
// }

// mod outer {
//     fn private() {
//         println!("outer::private");
//     }

//     pub fn public() {
//         println!("outer::public");
//     }

//     mod inner {
//         fn private() {
//             println!("outer::inner::private");
//         }

//         pub fn public() {
//             println!("outer::inner::public");
//             super::private();
//         }
//     }
// }

// #[rustfmt::skip]
fn main() {
    // let peter = Person {
    //     name: String::from("Peter"),
    //     age: 27,
    // };

    // println!("{} is {} years old", peter.name, peter.age);

    // let p = Point(17, 23);

    // println!("({}, {})", p.0, p.1);

    // let force = compute_thruster_force();
    // set_thruster_force(force);

    // let peter = Person::new(String::from("Peter"),27);
    // println!("{peter:?}");

    // println!("You got: {:?}",file_coin());

    // let load = WebEvent::PageLoad;
    // let press = WebEvent::KeyPress('x');
    // let click = WebEvent::Click { x: 20, y: 80 };

    // inspect(load);
    // inspect(press);
    // inspect(click);

    // dbg_size!(Foo);

    // let peter = Person {
    //     name: String::from("Peter"),
    //     age: 27,
    // };
    // peter.say_hello();

    // let mut race = Race::new("Monaco Grand Prix");

    // race.add_lap(70);
    // race.add_lap(68);
    // race.print_laps();
    // race.add_lap(71);
    // race.print_laps();
    // race.finsh();

    // let input = 'x';

    // match input {
    //     'q' => println!("Quitting"),
    //     'a' | 's' | 'w' | 'd' => println!("Moving around"),
    //     '0'..='9' => println!("Number input"),
    //     _ => println!("Something else"),
    // }

    // let n = 67;

    // match divide_in_two(n) {
    //     Result::Ok(half) => println!("{n} divided in two is {half}"),
    //     Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    // }

    // let foo = Foo {x : (3, 5), y: 123};

    // match foo {
    //     Foo {x: (1, b), y} => println!("x.0 = 1, b = {b}, y = {y}"),
    //     Foo {y : 2, x: i} => println!("y = 2, i = {i:?}"),
    //     Foo {y, ..} => println!("y = {y}, other fields were ignored"),
    // }

    // let triple = [5, -2, 3];

    // println!("Tell me about {triple:?}");

    // match triple {
    //     [0, y, z] => println!("First is 0, y = {y} z = {z}"),
    //     [1, ..] => println!("x = 1, other is ignored"),
    //     _ => println!("All elements were ignored"),
    // }

    // let pair = (2, 4);

    // println!("Tell me about {pair:?}");

    // match pair {
    //     (x, y) if x == y => println!("These are twins"),
    //     (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
    //     (x, _) if x % 2 == 1 => println!("The first one is odd"),
    //     _ => println!("No correlations"),
    // }

    // let bob = User::new(String::from("Bob"),32, 155.2);

    // println!("I'm {} and my age is {}", bob.name, bob.age);

    // let x = {
    //     let y = 10;
    //     println!("y : {y}");

    //     let z = {
    //         let w = {
    //             3 + 4
    //         };
    //         println!("w : {w}");
    //         y * w
    //     };
    //     println!("z: {z}");
    //     z - y
    // };
    // println!("x: {x}");

    // println!("doubled: {}", double(7));

    // let mut x = 10;
    // if x % 2 == 0 {
    //     x = x / 2;
    // }
    // else {
    //     x = 3 * x + 1;
    // }

    // let arg = std::env::args().next();
    // if let Some(value) = arg {
    //     println!("Program name : {value}");
    // }
    // else {
    //     println!("Missing name?");
    // }

    // let mut x = 10;

    // while x != 1 {
    //     x = if x % 2 == 0 {
    //         x / 2
    //     }
    //     else {
    //         3 * x + 1
    //     };
    // }
    // println!("Final x: {x}");

    // let v = vec![10, 20, 30];
    // let mut iter = v.into_iter();

    // while let Some(x) = iter.next() {
    //     println!("x : {x}");
    // }

    // let v = vec![10, 20, 30];

    // for x in v {
    //     println!("x: {x}");
    // }

    // let mut x = 10;

    // loop {
    //     x = if x % 2 == 0 {
    //         x / 2
    //     }
    //     else {
    //         3 * x + 1
    //     };
    //     if x == 1 {
    //         break;
    //     }
    // }

    // println!("Final x : {x}");

    // match std::env::args().next().as_deref() {
    //     Some("cat") => println!("Will do cat things"),
    //     Some("ls") => println!("Will ls some files"),
    //     Some("mv") => println!("Let's mov some files"),
    //     Some("rm") => println!("Uh, dangerous!"),
    //     None => println!("Hmm,no program name?"),
    //     _ => println!("Unknown program name!"),
    // }

    // let v = vec![10, 20, 30];

    // let mut iter = v.into_iter();

    // 'outer: while let Some(x) = iter.next() {
    //     println!("x: {x}");

    //     let mut i = 0;

    //     while i > x {
    //         println!("x: {x}, i: {i}");
    //         i += 1;
    //         if i == 3 {
    //             break 'outer;
    //         }
    //     }
    // }

    // let mut s1 = String::new();

    // s1.push_str("Hello");

    // println!("s1: len = {}, capacity = {}",s1.len(), s1.capacity());

    // let mut s2 = String::with_capacity(s1.len() + 1);
    // s2.push_str(&s1);
    // s2.push('!');

    // println!("s2: len = {},capaity = {}",s2.len(), s2.capacity());

    // let numbers = vec![10, 20, 30];

    // let first: Option<&i8> = numbers.first();

    // println!("first: {first:?}");

    // let idx: Result<usize, usize> = numbers.binary_search(&10);
    // println!("idx: {idx:?}");

    // let mut numbers = Vec::new();
    // numbers.push(42);

    // let mut v1 = Vec::new();
    // v1.push(42);
    // println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    // let mut v2 = Vec::with_capacity(v1.len() + 1);
    // v2.extend(v1.iter());
    // v2.push(9999);
    // println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    // let mut page_count = HashMap::new();

    // page_count.insert("Adventures of Huckleberry Finn".to_string(), 207);
    // page_count.insert("Grimms' Fairy Tales".to_string(), 751);
    // page_count.insert("Pride and Prejudice".to_string(), 303);

    // if !page_count.contains_key("Les Miserables") {
    //     println!("We've know about {} books, but not Les Miserables", 
    //             page_count.len());
    // }

    // for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
    //     match page_count.get(book) {
    //         Some(cnt) => println!("{book} : {cnt} pages"),
    //         None => println!("{book} is unknown"),
    //     }
    // }

    // let five = Box::new(5);
    // println!("five: {}", *five);

    // let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))

    // let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    // println!("{list:?}");

    // let mut a = Rc::new(10);
    // let mut b = a.clone();

    // println!("a: {a}");
    // println!("b: {b}");

    // foo::do_something();
    // bar::do_something();

    // outer::public();

}

// pub fn luhn(cc_number: &str) -> bool {
//     let test = cc_number.trim().parse::<f64>();

//     match test {
//         Ok(ok) => {
//             println!("123 : {ok}")
//         },
//         Err(err) => return false
//     }

//     false
// }

// #[test]
// fn test_non_digit_cc_number() {
//     assert!(!luhn("foo"));
// }

// fn double(x: i32) -> i32 {
//     x + x
// }

// #[test]
// fn test_weight() {
//     let bob = User::new(String::from("Bob"),32,155.2);
//     assert_eq!(bob.weight(), 155.2);
// }

// #[test] 
// fn test_set_age() {
//     let mut bob = User::new(String::from("Bob"),32, 155.2);
//     assert_eq!(bob.age(), 32);
//     bob.set_age(33);
//     assert_eq!(bob.age(), 33);
// }

// fn divide_in_two(n: i32) -> Result {
//     if n % 2 == 0 {
//         Result::Ok(n / 2)
//     }
//     else {
//         Result::Err(format!("cannot divide {} into two equal parts", n))
//     }
// }

// fn compute_thruster_force() -> PoundOfForce {
//     todo!("Ask a rocket scientist as NASA")
// }

// fn set_thruster_force(force: Newtons) {
//     println!("{?}", force);
// }

// fn generate_random_number() -> i32 {
//     4
// }

// fn file_coin() -> CoinFlip {
//     let random_number = generate_random_number();
//     if random_number % 2 == 0 {
//         return CoinFlip::Heads;
//     }
//     else {
//         return CoinFlip::Tails;
//     }
// }

// #[rustfmt::skip]
// fn inspect(event: WebEvent) {
//     match event {
//         WebEvent::PageLoad => println!("page loaded"),
//         WebEvent::KeyPress(c) => println!("pressed '{c}'"),
//         WebEvent::Click { x, y} => println!("clicked at x={x}, y={y}"),
//     }
// }