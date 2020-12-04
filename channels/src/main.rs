#[path = "channel_clone_emisor.rs"] mod example_1;
#[path = "many_productors.rs"] mod example_2;


fn main() {
    println!("================EXAMPLE 1==================");
    example_1::clone_emisor_various_values();
    println!("================EXAMPLE 2==================");
    example_2::create_various_productors_and_clone_transmisor();

}