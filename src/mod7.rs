pub fn nst_prime(n: u64) -> u64{
    
    let mut primes: Vec<u64>= Vec::new();
    let mut current: u64 = 2;    
    while primes.len() < n.try_into().unwrap() {
       let is_prime = primes
            .iter()
            .take_while(|&x| x*x <= current )
            .all(|&x| current % x != 0);
        if is_prime {
            primes.push(current);
        }
        match current {
            2 => current += 1,
            _ => current += 2,
        }
    }
    
    let nst_prime_number: u64 = primes.pop().expect("Nao conseguimos obter o numero primo nessa posicao");
    
    println!("O numero primo {} eh {} ",n,nst_prime_number);

    return nst_prime_number;
    

}
