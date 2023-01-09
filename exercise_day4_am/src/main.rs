// use std::fmt::format;
// use std::thread;
// use std::sync::Arc;
// use std::time::Duration;
// use std::sync::mpsc;
// use std::sync::Mutex;

fn main() {
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Count in thread : {i}!");
    //         thread::sleep(Duration::from_millis(5));
    //     }
    // });

    // for i in 1..5 {
    //     println!("Main thread: {i}");
    //     thread::sleep(Duration::from_millis(5));
    // }

    // let s = String::from("Hello");

    // thread::spawn(|| {
    //     println!("Length: {}", s.len());
    // });

    // let s = String::from("Hello");

    // thread::scope(|scope| {
    //     scope.spawn(|| {
    //         println!("Length: {}", s.len());
    //     });
    // });

    // let (tx, rx) = mpsc::channel();

    // tx.send(10).unwrap();
    // tx.send(20).unwrap();

    // println!("Received: {:?}", rx.recv());
    // println!("Received: {:?}", rx.recv());

    // let tx2 = tx.clone();
    // tx2.send(30).unwrap();
    // println!("Received: {:?}", rx.recv());

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let thread_id = thread::current().id();
    //     for i in 1..10 {
    //         tx.send(format!("Message {i}")).unwrap();
    //         println!("{thread_id:?}: sent message {i}");
    //     }
    //     println!("{thread_id:?} : done");
    // });

    // thread::sleep(Duration::from_millis(100));

    // for msg in rx.iter() {
    //     println!("Main: got {}", msg);
    // }

    // let (tx, rx) = mpsc::sync_channel(3);

    // thread::spawn(move || {
    //     let thread_id = thread::current().id();

    //     for i in 1..10 {
    //         tx.send(format!("Message: {i}")).unwrap();
    //         println!("{thread_id:?} : send Message : {i}");
    //     }
    //     println!("{thread_id:?} : done");
    // });

    // thread::sleep(Duration::from_millis(100));

    // for msg in rx.iter() {
    //     println!("Main: got {}", msg);
    // }

    // let v = Arc::new(vec![10, 20, 30]);
    // let mut handles = Vec::new();

    // for _ in 1..5 {
    //     let v = v.clone();
    //     handles.push(thread::spawn(move || {
    //         let thread_id = thread::current().id();
    //         println!("{thread_id:?}: {v:?}");
    //     }));
    // }
    // handles.into_iter().for_each(|h| h.join().unwrap());
    // println!("v: {v:?}");

    // let v: Mutex<Vec<i32>> = Mutex::new(vec![10, 20, 30]);

    // println!("v: {:?}", v.lock().unwrap());

    // {
    //     let v: &Mutex<Vec<i32>> = &v;
    //     let mut guard = v.lock().unwrap();
    // }

    // println!("v: {:?}", v.lock().unwrap());

    // let v:Mutex<Vec<i32>> = Mutex::new(vec![10, 20, 30]);

    // let handle = thread::scope(|scope| {
    //     thread::spawn(|| {
    //         let v: &Mutex<Vec<i32>> = &v;
    //         let mut guard = v.lock().unwrap();
    //         guard.push(10);
    //     });
    // });

    // v.lock().unwrap().push(1000);

    // handle.join().unwrap();
    // println!("v: {v:?}");

}
