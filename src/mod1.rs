pub fn multiples_of_3_or_5() -> (){
    let sum_of_multiples: u32 = (1..1000)
        .filter(|x| { x%5 == 0 || x%3 == 0})
        .sum();

        println!("The sum is {}",sum_of_multiples);

}
