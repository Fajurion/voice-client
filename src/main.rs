use std::{sync::{Mutex, Arc}};

mod record;

fn main() {

    let vec = Arc::new(Mutex::new(vec![1, 2, 3]));
    let v2 = vec.clone();

    let closure = move || {
        v2.lock().unwrap().push(4);
    };

    closure();

    let final_x = vec.lock().unwrap();
    println!("{:?}", *final_x); // prints [1, 2, 3, 4]


    record::record_microphone();
}
