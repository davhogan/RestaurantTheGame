mod employee;
mod menu_items;
mod names;

pub use names::*;
pub use employee::*;
pub use menu_items::*;

use names::{Generator, Name};

fn main() {
    let mut generator = Generator::with_naming(Name::Plain);

    for i in 0..3 {
        println!("Name is: {}", generator.next().unwrap());
    }



}
