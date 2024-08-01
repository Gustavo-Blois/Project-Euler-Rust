pub fn largest_palindrome_product() -> () {
    let numbers: Vec<u32> = (100..1000).rev().collect();
    let mut largest_palindrome = 0;
    for i in &numbers {
        for j in &numbers {
            let n: u32 = i*j;
            if n.to_string().chars().rev().collect::<String>() == n.to_string() && n > largest_palindrome {
    largest_palindrome = n;
            }
        }
    }

    println!("The largest_palindrome_product is {}",largest_palindrome);

}
