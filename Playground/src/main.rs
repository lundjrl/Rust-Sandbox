mod examples;

use examples::add::add;
use examples::data_types::data_types;

fn main() {
    const RESULT: u8 = add(2, 3);
    println!("{RESULT}");

    let x = data_types();
    println!("{x}");
}
