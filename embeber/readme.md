## Rust embebido con python y nodejs
<img src="https://i.morioh.com/2020/04/29/2ae76f20b377.jpg">
<img src="https://i.morioh.com/200724/cd9eb82c.webp">

1) Agregar en el archivo Cargo.toml

```
  [lib]
  name = "embeber"
  crate-type = ["cdylib"]
```
Esto le dice a cargo que queremos hacer una biblioteca dinámica compatible con C ( `crate-type = ["cdylib"]`) y cómo llamarla, además de algunos metadatos estándar. Luego podemos poner nuestro código en formato src/lib.rs.

Usaremos dos funciónes de juguete simples, una para procesar multiples hilos, sumar y restar numeros:

```rust
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
  pub fn add(a: i64, b: i64) -> i64 {
    a + b
  }
```

Observe la palabra clave `pub`, que le indica al compilador que haga que esta función sea accesible para otros módulos, y la `#[no_mangle]` anotación, que le dice que use las convenciones de nomenclatura estándar de C para las funciones. Si no hacemos esto, entonces Rust generará un nuevo nombre para la función para sus propios propósitos nefastos, y como efecto secundario no sabremos cómo llamarlo cuando queramos usarlo desde Python.


Ejecutamos:

```bash
  $cargo build --release
```

### El bit de Python
Después de todo eso, el bit de Python es bastante corto. Primero importamos el ctypespaquete (que se incluye en todas las versiones recientes de Python):

```python
  from ctypes import cdll
```

Cargo ha ordenado nuestra biblioteca compartida en una carpeta, por lo que debemos decirle a Python desde dónde cargarla. En Linux, se llamará `lib<something>.so` donde el "algo" es el nombre de la caja Cargo.toml, "embeber":

```
  lib = cdll.LoadLibrary('target/release/libembeber.so')
```

Finalmente podemos llamar a la función en cualquier lugar que queramos. Aquí está en una prueba de estilo pytest:

```python
  def test_rust_add():
    assert lib.add(27, 15) == 42
```

Si tiene pytest instalado (¡y debería hacerlo!), Puede ejecutar toda la prueba de esta manera:

```bash
  $pytest --verbose test.py
```
