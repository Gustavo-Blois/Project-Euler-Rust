pub fn largest_prime_factor() -> () {
    let mut number: u64 = 600851475143;
    let mut i: u64 = 2;
    let mut largest = 2;
    while i*i <= number {
        if number % i == 0 {
            number /= i;
            largest = i;
        }
        else {
            i +=1;
        }
    }

    if number > 1 {
        largest = number;
    }

    println!("O maior fator primo é {}",largest);
}
