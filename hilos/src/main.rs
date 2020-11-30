use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        "Hello from one thread, Heyyyyyyyyy! :)"
    });

    println!("{}", handle.join().unwrap());
}


