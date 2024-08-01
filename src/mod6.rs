pub fn sum_square_difference() -> () {
    let list = 1..=100;
    let sum_of_squares: i64 = list.clone().map(|x| x.square()).sum();
    let squares_of_sum: i64 = list.clone().sum::<i64>().square();
    let difference = squares_of_sum - sum_of_squares;
    println!("The difference is {}",difference);
}

trait Square {
    fn square(&self) -> i64;
}

impl Square for i64 {
    fn square(&self) -> i64 {
        self*self
    }
}

