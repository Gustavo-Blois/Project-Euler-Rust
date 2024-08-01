pub fn summation_of_primes() {
    let mut primes = vec![true;2000001];
    let mut sum = 0;
    let mut p = 2;

    while p <= 2000000 {
        if primes[p] {
            let mut i = p * p;
            sum += p;
            while i <= 2000000 {
                primes[i] = false;
                i += p;
            }
        }
        p += 1;
    }
    println!("A soma dos primos menores que 2000000 eh {}",sum);
}
