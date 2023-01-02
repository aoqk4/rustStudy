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

    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    // for word in &v {
    //     println!("word: {word}");
    // }

    // for word in v {
    //     println!("word: {word}");
    // }

    for word in v.iter() {
        println!("word : {word}");
    }

    for word in v.into_iter() {
        println!("word : {word}");
    }


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