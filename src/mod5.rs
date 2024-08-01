pub fn smallest_multiple() -> () {
    let mut current = 2520;
    loop {
        
        let dividable = |current| {
            for i in 11..=20 {
                if current % i != 0 {
                    return false; 
                }
            }
            return true;
        };
        if dividable(current){
            println!("O menor múltiplo dos números de 1 a 20 é {}",current);
            break;
        }
        current += 2520;
    }
}

