// use std::io::{BufRead, BufReader, Read, Result, Write};
// use std::{fmt::Display, vec};


// trait Greet {
//     fn say_hello(&self);
// }

// struct Dog {
//     name: String,
// }

// struct Cat;

// impl Greet for Dog {
//     fn say_hello(&self) {
//         println!("Wuf, my name is {}!", self.name);
//     }
// }

// impl Greet for Cat {
//     fn say_hello(&self) {
//         println!("Miau!");
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Default)]
// struct Player {
//     name: String,
//     strength: u8,
//     hit_points: u8,
// }

// trait Equals {
//     fn equal(&self, other:&Self) -> bool;

//     fn not_equal(&self, other:&Self) -> bool {
//         !self.equal(other)
//     }
// }

// #[derive(Debug)]
// struct Centimeter(i16);

// impl Equals for Centimeter {
//     fn equal(&self, other:&Self) -> bool {
//         self.0 == other.0
//     }
// }

// struct Fibonacci {
//     curr: u32,
//     next: u32,
// }

// impl Iterator for Fibonacci {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         let new_next = self.curr + self.next;
//         self.curr = self.next;
//         self.next = new_next;

//         Some(self.curr)
//     }
// }

// #[derive(Debug, Clone, Copy)]
// struct Point {x: i32, y:i32}

// impl std::ops::Add for Point {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self {
//         Self {x: self.x + rhs.x, y : self.y + rhs.y}
//     }
// }

// struct Droppable {
//     name: &'static str,
// }

// impl Drop for Droppable {
//     fn drop(&mut self) {
//         println!("Dropping {}", self.name);
//     }
// }

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// #[derive(Debug)]
// struct Point<T> (T, T);

// impl <T> Point<T> {
//     fn x(&self) -> &T {
//         &self.0
//     }
// }

// fn main() {
// fn main() -> Result<()> {
    // let pets: Vec<Box<dyn Greet>> = vec![
    //     Box::new(Dog { name: String::from("Fido")}),
    //     Box::new(Cat),
    // ];
    // for pet in pets {
    //     pet.say_hello();
    // }

    // let p1 = Player::default();
    // let p2 = p1.clone();

    // println!("Is {:?}\nequal to {:?}?\nThe answer is {}!", &p1, &p2,
    //         if p1 == p2 {"Yes"} else {"No"});

    // let a = Centimeter(10);
    // let b = Centimeter(20);

    // println!("{a:?} equals {b:?}: {}",a.equal(&b));
    // println!("{a:?} not_equals {b:?}: {}",a.not_equal(&b));

    // let fib = Fibonacci{curr : 0, next : 1};
    // for (i, n) in fib.enumerate().take(30) {
    //     println!("fib({i}) : {n}");
    // }

    // let s = String::from("hello");
    // let addr = std::net::IpAddr::from([127, 0, 0, 1]);
    // let one = i16::from(true);
    // let bigger = i32::from(123116);

    // let s: String = "hello".into();
    // let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    // let one: i16 = true.into();
    // let bigger: i32 = 12316.into();

    // println!("{s}, {addr}, {one}, {bigger}");

    // let slice: &[u8] = b"foo\nbar\nbaz\n";
    // println!("lines in slice: {}", count_lines(slice));

    // let file = std::fs::File::open(std::env::current_exe()?)?;
    // println!("lines in file: {}", count_lines(file));
    // Ok(())

    // let mut buffer = Vec::with_capacity(1024);
    // log(&mut buffer, "Hello")?;
    // log(&mut buffer, "World")?;
    // println!("Logged: {:?}", buffer);
    // Ok(())

    // let p1 = Point { x: 10, y: 20 };
    // let p2 = Point { x: 100, y: 200 };

    // println!("{:?} + {:?} = {:?}",p1, p2, p1 + p2);

    // let a = Droppable {name : "a"};
    // {
    //     let b = Droppable { name : "b" };
    //     {
    //         let c = Droppable { name : "c" };
    //         let d = Droppable { name : "d" };
    //         println!("Exiting block B");
    //     }
    //     println!("Exiting block A");
    // }
    // drop(a);
    // println!("Exiting main");

    // let integer = Point {x : 5, y: 10};
    // let float = Point {x : 1.0, y: 4.0};

    // println!("{integer:?} and {float:?}");

    // let p = Point(5, 10);
    // println!("p.x = {}", p.x());

    // let foo = String::from("foo");
    // let pair = duplicate(foo);

    // println!("{:?}",pair);

    // let add_3 = |x| x + 3;
    // let mul_5 = |x| x * 5;

    // println!("add_3 : {}", apply_with_log(add_3, 10));
    // println!("add_3 : {}", apply_with_log(add_3, 10));

    // let integer = Some(5);
    // let float = Some(5.0);

    // print(123);
    // print("Hello");

    // let xs = vec![123, "Hello"];
    // println!("{:?}",xs);

    // let xs: Vec<Box<dyn Display>> = vec![Box::new(1234), Box::new("Hello")];
    // for x in xs {
    //     println!("x: {x}");
    // }

//     println!("{:?}", numbers(-5).collect::<Vec<_>>());
//     println!("{:?}", numbers(5).collect::<Vec<_>>());

// }

// fn numbers(n: i32) -> Box<dyn Iterator<Item = i32>> {
//     if n > 0 {
//         Box::new(0..n)
//     }
//     else {
//         Box::new((n..0).rev())
//     }
// }

// fn print<T: Display>(x: T) {
//     println!("Your value: {}", x);
// }

// fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
//     println!("Calling function on {input}");
//     func(input)
// }

// fn duplicate<T: Clone>(a: T) -> (T, T) {
//     (a.clone(), a.clone())
// }

// fn count_lines<R: Read>(reader: R) -> usize {
//     let buf_reader = BufReader::new(reader);
//     buf_reader.lines().count()
// }

// fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
//     writer.write_all(msg.as_bytes())?;
//     writer.write_all("\n".as_bytes())
// }

fn main() {

}