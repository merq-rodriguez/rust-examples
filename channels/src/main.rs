use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    for _ in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            let respuesta = 42u32;
            tx.send(respuesta);
        });
    }
    
    rx.recv()
        .ok()
        .expect("No se ha podido recibir la respuesta");
}
