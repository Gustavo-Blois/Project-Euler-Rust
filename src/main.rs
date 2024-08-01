use std::{
    env,
    error::Error,
};

mod mod1;
mod mod2;
mod mod3;
mod mod4;
mod mod5;
mod mod6;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:");
        println!("eulerust: <challenge>");
        return Ok(());
    }

    match args[1].trim() {
        "6" => mod6::sum_square_difference(),
        "5" => mod5::smallest_multiple(),
        "4" => mod4::largest_palindrome_product(),
        "3" => mod3::largest_prime_factor(),
        "2" => mod2::even_fibonacci(),
        "1" => mod1::multiples_of_3_or_5(),  
          _ => 
           println!("Essa opção não é válida"),
    }
    
    Ok(())
}
