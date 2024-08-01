pub fn even_fibonacci() -> () {
    let mut prev1 = 1;
    let mut prev2 = 2;
    let mut current = 0;
    let mut sum = 2;
    while current < 4000000 {
        current = prev1+prev2;
        prev1 = prev2;
        prev2 = current;
        if current % 2 == 0 {
            sum += current;
        }
    }

    println!("The sum of the first 4000000 even fibonnaci numbers is {}",sum);
}
