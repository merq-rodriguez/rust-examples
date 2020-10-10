use std::thread;

#[no_mangle]
pub extern fn procesar(){
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut _x = 0;
            for _ in 0..5000000 {
                _x += 1
            }
        })
    }).collect();

    for h in handles {
        h.join()
            .ok()
            .expect("No se pudo unir un hilo");
    }
}

#[no_mangle]
pub extern fn suma(a: i64, b: i64) -> i64 {
    a + b
}

#[no_mangle]
pub extern fn resta(a: i64, b: i64) -> i64 {
    a - b
}