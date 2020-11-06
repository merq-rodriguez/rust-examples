

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
    println!(" ===================== Find ===================== ");
    
    let mayores_a_cuarenta = (1..100).find(|x| *x > 40);

    match mayores_a_cuarenta {
        Some(_) => println!("Tenemos algunos numeros!"),
        None => println!("No se encontraron numeros :("),
    }

    println!(" ===================== Fold ===================== ");


    //FOLD
    let suma = (1..4).fold(0, |suma, x| suma + x);
    println!("Suma: {}", suma);

    println!("Filter");

    println!(" ===================== Iter ===================== ");
    let nums = vec![1,2,3,4];

    for num in nums.iter(){
        println!("{}", num)
    }
    
    println!(" ===================== Map ===================== ");
    let mut nums2 = (1..10).map(|x| x + 1);
    for i in nums2{
        println!("{}", i);
    }

    println!(" ===================== take ===================== ");
    for i in (1..).take(5){
        println!("{}", i);
    }

    println!(" ===================== Filter ===================== ");
    for i in (1..50).filter(|&x| x % 2 == 0){
        println!("{}", i);
    }
    println!(" ===================== Filter ===================== ");
    for i in (1..)
               .filter(|&x| x % 2 == 0)
               .filter(|&x| x % 3 == 0)
               .take(5)
               .collect::<Vec<i32>>(){
                   println!("{}", i);
               }
}
