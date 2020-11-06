

fn main() {
    let mut rango = 1..11;
    let mut acum = 0;
    loop{
        match rango.next(){
            Some(x) => {
                println!("{} iteracion", x);
                acum = acum + x;
            },
            None =>  break 
        }
    }
    println!("Total: {}", acum);


    // CONSUMIDORES
    // FIND
    let mayores_a_cuarenta = (1..100).find(|x| *x > 40);

    match mayores_a_cuarenta {
        Some(_) => println!("Tenemos algunos numeros!"),
        None => println!("No se encontraron numeros :("),
    }

    //FOLD
    let suma = (1..4).fold(0, |suma, x| suma + x);
    println!("Suma: {}", suma);

    //ITER
    let nums = vec![1,2,3,4];

    for num in nums.iter(){
        println!("{}", num)
    }
}
