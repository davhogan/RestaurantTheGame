// Copyright © 2019 David Hogan
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

#[macro_use]
extern crate rand;
#[macro_use]
extern crate text_io;
mod ui;
use ui::UI;
//mod simulator;
//use simulator::Simulator;

fn main() {

    let mut choice : String = " ".to_owned();
    
    println!("Welcome to Restaurant The Game");
    println!("This is a text based restaurant simulation game");
    println!("You will be in charge of managing a restaurant.");
    println!("This includes hiring and firing employees, ordering more inventory,");
    println!("and changing the price and quality of the food on the menu.");
    println!("Press q to quit or enter to launch game");
    
    choice = read!();
    if choice == "q".to_owned() {
        return ;
    }

    let mut ui = UI::new();
    ui.home_page();

}
