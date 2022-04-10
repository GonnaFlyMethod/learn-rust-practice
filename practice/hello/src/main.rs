use rand::Rng;
use hello::greet;

fn main() {
    greet();
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    println!("{}", y);
}
