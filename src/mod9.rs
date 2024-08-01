pub fn special_pythagorean_triplet() {

    for n in 1..50 {
        for m in (1 + n)..50 {
        
        
        let a:i32 = (m as i32).pow(2) - (n as i32).pow(2);
        let b:i32 = 2*m*n;
        let c:i32 = (m as i32).pow(2) + (n as i32).pow(2);
        
        if a + b + c == 1000 {
            println!("A tripla pitagorica {} {} e {} somadas sao igual a 1000 e multiplicadas sao igual a {}",a,b,c,a*b*c);
            break;
        }
    }
    }

    
}
