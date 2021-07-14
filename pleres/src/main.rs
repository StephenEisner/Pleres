extern crate bluer;
use bluer::Session;

fn main() {
    println!("Hello, world!");
    let blue_sess = Session::new().await();
    //let adapter = blue_sess.adapter("");
}



