// use std::panic;
// use std::{fs, io, error};
// use std::io::Read;
// use thiserror::Error;
// use anyhow::{Context, Result};
// use std::fmt::format;

// #[derive(Debug)]
// enum ReadUsernameError {
//     IoError(io::Error),
//     EmptyUsername(String),
// }

// impl From<io::Error> for ReadUsernameError {
//     fn from(err: io::Error) -> ReadUsernameError {
//         ReadUsernameError::IoError(err)
//     }
// }

// #[derive(Error, Debug)]
// enum ReadUsernameError {
//     #[error("Cloud not read: {0}")]
//     IoError(#[from] io::Error),
//     #[error("Found no username in {0}")]
//     EmptyUsername(String),
// }

// #[derive(Error, Debug)]
// enum ReadUsernameError {
//     #[error("Could not read: {0}")]
//     IoError(#[from] io::Error),
//     #[error("Found no username in {0}")]
//     EmptyUsername(String),
// }

// static mut COUNTER: u32 = 0;

// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// #[repr(C)]
// union MyUnion {
//     i: u8,
//     b: bool,
// }

fn main() {
    // let v = vec![10, 20, 30];

    // println!("v[100] : {}", v[100]);

    // let result = panic::catch_unwind(|| {
    //     println!("hello!");
    // });

    // assert!(result.is_ok());

    // let result = panic::catch_unwind(|| {
    //     panic!("oh, no");
    // });

    // assert!(result.is_err());

    // let file = File::open("diary.txt");

    // match file {
    //     Ok(mut file) => {
    //         let mut contents = String::new();
    //         file.read_to_string(&mut contents);
    //         println!("Dear Diary: {contents}");
    //     },
    //     Err(err) => {
    //         println!("The diary could not be opened: {err}");
    //     }
    // }

    // match some_expression {
    //     Ok(value) => value,
    //     Err(err) => return Err(err),
    // }

    // let useranme = read_username("config.dat");
    // println!("username: {useranme:?}");

    // match exprssion {
    //     Ok(value) => value,
    //     Err(err) => return Err(From::from(err)),
    // }

    // let username = read_username("config.dat");
    // println!("username: {username:?}");

    // match read_username("config.dat") {
    //     Ok(username) => println!("Username: {username}"),
    //     Err(err) => println!("Error: {err}"),
    // }

    // match read_username("config.dat") {
    //     Ok(username) => println!("Username: {username}"),
    //     Err(err) => println!("Error : {err:?}"),
    // }

    // println!("{}", helper("Hello", "World!"));

    // let mut num = 5;

    // let r1 = &mut num as *mut i32;
    // let r2 = &num as *const i32;

    // unsafe {
    //     println!("r1 is: {}", *r1);
    //     *r1 = 10;
    //     println!("r1 is: {}", *r2);
    // }

    // static HELLO_WORLD: &str = "Hello world!";

    // println!("name is {}", HELLO_WORLD);

    // add_to_counter(42);

    // unsafe {
    //     println!("COUNTER: {}", COUNTER);
    // }

    // let emojis = "🗻∈🌏";

    // unsafe {
    //     println!("{}", emojis.get_unchecked(0..4));
    //     println!("{}", emojis.get_unchecked(4..7));
    //     println!("{}", emojis.get_unchecked(7..11));
    // }

    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    // let u = MyUnion{ i : 42 };
    // println!("int: {}", unsafe {
    //     u.i
    // });
    // println!("int: {}", unsafe {
    //     u.b
    // });

}

// fn add_to_counter(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }

// /// Shorten string will trip the string to given length
// /// ```
// /// use playground::shorten_string;
// /// assert_eq!(shorten_string("Hello World", 5),"Hello");
// /// assert_eq!(shorten_string("Hello World", 5),"Hello World");
// /// ```

// pub fn shorten_string(s: &str, length: usize) -> &str {
//     &s[..std::cmp::min(length, s.len())]
// }

// fn helper(a: &str, b:&str) -> String {
//     format!("{a} {b}")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_helper() {
//         assert_eq!(helper("foo", "bar"), "foo bar");
//     }
// }

// fn first_word(text: &str) -> &str {
//     match text.find(' ') {
//         Some(idx) => &text[..idx],
//         None => &text,
//     }
// }

// #[test]
// fn test_empty() {
//     assert_eq!(first_word(""), "");
// }

// #[test] 
// fn test_single_word() {
//     assert_eq!(first_word("Hello"), "Hello");
// }

// #[test]
// fn test_multiple_words() {
//     assert_eq!(first_word("Hello World"), "Hello");
// }

// fn read_username(path: &str) -> Result<String> {
//     let mut username = String::with_capacity(100);
//     fs::File::open(path)
//         .context(format!("Failed to open {path}"))?
//         .read_to_string(&mut username)
//         .context("Fail to read")?;
//     if username.is_empty() {
//         return Err(ReadUsernameError::EmptyUsername(String::from(path)))?;
//     }
//     Ok(username)
// }

// fn read_username(path: &str) -> Result<String, ReadUsernameError> {
//     let mut username = String::with_capacity(100);
//     fs::File::open(path)?.read_to_string(&mut username)?;

//     if username.is_empty() {
//         return Err(ReadUsernameError::EmptyUsername(String::from(path)));
//     }
//     Ok(username)
// }

// fn read_username(path: &str) -> Result<String, ReadUsernameError> {
//     let mut username = String::with_capacity(100);

//     fs::File::open(path)?.read_to_string(&mut username)?;

//     if username.is_empty() {
//         return Err(ReadUsernameError::EmptyUsername(String::from(path)));
//     }
//     Ok(username)
// }

// fn read_username(path: &str) -> Result<String, io::Error> {
//     let username_file_result = fs::File::open(path);

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
