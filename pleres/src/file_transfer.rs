use std::env;
use std::fs::File;

fn connect(ext_dev_handle: &'static str) -> Result<Connection, &'static str>{

}

fn send(ext_dev_handle: &'static str, f_handle: File) -> Result<TransactionStatus, &'static str>{

}

fn daemonize(){ 

}

fn main() {
    let args: Vec<String> = env::args().collect();
}

