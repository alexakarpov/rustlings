// use eat_at_restaurant;

// fn main() {
//     eat_at_restaurant();
//     println!("Hello world!");
// }

use restaurant::eat_at_restaurant;
use restaurant::GREETING;
fn main() {
    println!("{}", GREETING);

    eat_at_restaurant();
}
