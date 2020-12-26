use rand::prelude::*;

fn main() {
    // infer by context to be generates a boolean
    if rand::random() {
        println!("char: {}", rand::random::<char>());
    }
    println!("{}", rand::random::<bool>());
    let mut rng = rand::thread_rng();
    // generates a float between 0 and 1
    let y: f64 = rng.gen();
    println!("y = {}", y);

    let mut nums: Vec<i32> = (1..10).collect();
    // shuffle algorithms
    nums.shuffle(&mut rng);
    println!("{:?}", nums)
}
