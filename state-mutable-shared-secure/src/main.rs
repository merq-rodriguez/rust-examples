use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main(){
    let  data = Arc::new(Mutex::new(vec![1u32, 2, 3]));

    for i in 0..3{
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }

    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
}
