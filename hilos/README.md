## Hilos

El metodo `thread::spawn()` acepta un closure como argumento, closure que es ejecutado en un nuevo hilo.

`thread::spawn()` retorna un handler a el nuevo hilo, que puede ser usado para esperar a que el hilo finalice y luego extraer su resultado:


```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        "Hello from one thread, Heyyyyyyyyy! :)"
    });

    println!("{}", handle.join().unwrap());
}
```

## Estado Mutable Compartido Seguro

