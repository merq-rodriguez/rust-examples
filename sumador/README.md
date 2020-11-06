## Iteradores
 
Un iterador es algo en lo que podemos llamar al metodo `.next()` repetitivamente, y el iterador nos proporciona una secuencia de elementos.


```rust
  let mut rango = 0..10;
  loop {
    match rango.next() {
      Some(x) => {
        println!("{}", x);
      },
      None => { break }
    }
  }
```
## Consumidores

Un consumidor opera en un iterador, retornando algún tipo de valor o valores. El consumidor mas común es `collect()`, pero hay otros como `find()`:

```rust
  let mayores_a_cuarenta = (1..100).find(|x| *x > 40);

  match mayores_a_cuarenta {
    Some(_) => println!("Tenemos algunos numeros!"),
    None => println!("No se encontraron numeros :("),
  }

```

`find` recibe un closure, y trabaja en una referencia a cada elemento de un iterador. Dicho closure retorna `true` si el elemento es el que estamos buscando y `false` de lo contrario. Debido a que podríamos no encontrar un elemento que satisfaga nuestro criterio, `find` retorna un `Option` en lugar de un elemento.

Otro consumidor importante es `fold`. Luce asi:

```rust
  let suma = (1..4).fold(0, |suma, x| suma + x);
```

Como hemos dicho antes, un iterador es algo en lo que podemos llamar el método `.next()` repetidamente, y este nos devuelve una secuencia de elementos. Debido a que necesitamos llamar a el método, los iteradores pueden ser perezosos y no generar todos los valores por adelantado. Este código, por ejemplo, no genera los números 1-99. En su lugar crea un valor que representa la secuencia:

```rust
let nums = 1..100
```
Debido a que no hicimos nada con el rango, este no genero la secuencia. Agreguemos unconsumidor:

```rust
  let nums = (1..100).collect::<Vec<i32>>();
```

`iter()` puede transformar un vector en un iterador simple que proporciona un elemento a la vez:

```rust
let nums = vec![1,2,3,4];

for num in nums.iter(){
    println!("{}", num)
}
```
