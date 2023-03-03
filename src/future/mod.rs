use futures::executor::block_on;
use std::{thread, time::Duration};

pub mod pin;

async fn hello_world() {
    // thread::sleep(Duration::from_secs(3));
    println!("hello, world!");
}

async fn hello_world2() {
    // thread::sleep(Duration::from_secs(2));
    println!("hello, world2222!");
}

async fn hello_world3() {
    // thread::sleep(Duration::from_secs(2));
    println!("hello, world2222asdasdasd!");
}

async fn do_all() {
    let future = hello_world(); // Nothing is printed
    let future2 = hello_world2(); // Nothing is printed
    let future3 = hello_world3(); // Nothing is printed

    futures::join!(future, future2, future3);
}

#[test]
fn test_main() {
    block_on(do_all());
    // futures::join!(future, future2, future3);
    // println!("asd");
}
