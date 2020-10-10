#Concurrencia

Aun así son solo cinco lineas, son cinco densas lineas. Analicemos por partes.let handles: 

```
  Vec<_> =
```

Introducimos una nueva variable, llamadahandles. Le hemos dado este nombre porquecrearemos algunos nuevos hilos, que resultaran en algunos handles (agarradores, manillas)a esos dichos hilos los cuales nos permitirán controlar su operación. Necesitamos anotar eltipo explícitamente, debido a algo que haremos referencia mas adelante. 

Elmarcador de posición para un tipo. Estamos diciendo “_es unes un vector de algo, perohandlestu, Rust, puedes determinar que es ese algo.”

```
  filosofos.into_iter().map(|f| {
```

Tomamos nuestra lista de filósofos y llamamos en ella. Esto crea un iterador into_iter() que se adueña (toma pertenencia) de cada filosofo. Necesitamos hacer esto para poder pasar los filósofos a nuestros hilos. Luego tomamos ese iterador y llamamos `map` en el,método que toma un closure como argumento y llama dicho closure en cada uno de los elementos a la vez.

```
  thread::spawn(move || {
    f.comer();
  })
```

Es aquí donde la concurrencia ocurre. La funciónthread::spawntoma un closure comoargumento y ejecuta ese closure en un nuevo hilo. El closure necesita una anotación extra, `move`, para indicar que el closure va a adueñarse de los valores que esta capturando. Principalmente, la variable `f` de la función `map`. 

Dentro del hilo, todo lo que hacemos es llamar a `commer();` en `f`.

```
   }).collect();
```

Finalmente, tomamos el resultado de todos esas llamadas a `map`. `collect()` y los coleccionamos. los convertirá en una colección de algun tipo, que es el porque anotamos el tipo de retorno: queremos un `Vec<T>`. Los elementos son los valores retornados de las llamadas a `thread::spawn`, que son handles a esos hilos. Whew!

```rust
  for h in handles {
    h.join().unwrap();
  }
```

Al final de `main()`, iteramos a través de los handles llamando `join()` en ellos, lo cual bloquea la ejecución hasta que el hilo haya completado su ejecución. Esto asegura que el hilo complete su ejecución antes que el programa termine. Si ejecutas este programa, veras que los filósofos comen sin orden! Tenemos multi-hilos!

```
Gilles Deleuze esta comiendo.Gilles Deleuze ha finalizado de comer.Emma Goldman esta comiendo.Emma Goldman ha finalizado de comer.Michel Foucault esta comiendo.Judith Butler esta comiendo.Judith Butler ha finalizado de comer.Karl Marx esta comiendo.Karl Marx ha finalizado de comer.Michel Foucault ha finalizado de comer.
```

### Mesa
```rust
  use std::sync::Mutex;
  
  struct Mesa {
    tenedores: Vec<Mutex<()>>,
  }
```

Esta Mesa contiene un vector de Mutexes. Un mutex es una forma de controlar concurrencia, solo un hilo puede acceder el contenido a la vez. Esta es la exactamente la propiedad que necesitamos para nuestros tenedores. Usamos una dupla vacía,(), dentro del mutex, debido a que no vamos a usar el valor, solo nos aferraremos a el.

Modifiquemos el programa para hacer uso de Mesa:

