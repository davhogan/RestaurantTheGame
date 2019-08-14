// Copyright © 2019 David Hogan
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.


mod simulator;
use simulator::Simulator;

// Used to interact with the user.
// Handles taking in most inputs from the user.
// Has a home_page function to serve as the main hub for the user.
// Handles the inputs of the user to accomplish the management of the restaurant.
// Most of the function calls made are to the simulator to update the restaurant.

pub struct UI {
    sim: Simulator,
}

impl UI {
    pub fn new() -> UI {
        let sim = Simulator::new();
        UI { sim }
    }
    
    //The main display for the user to interact with.
    pub fn home_page(&mut self) {
        let mut choice : i64 = -1;
        if self.sim.get_day()%7 == 0 {
            self.sim.update_pot();
        }
        println!("{}",self.sim.get_name());
        println!("Current Day {}", self.sim.get_day()+1);
        println!("Current Revenue : ${}", self.sim.get_revenue());
        
        while choice < 1 || choice > 7 {
            println!("[1] Display Menu\n[2] Display Employees\n[3] Hire Employee\n[4] Fire Employee");
            println!("[5] Order Menu Item\n[6] Change Menu Item Price\n[7] Go To Next Day\n[8] Quit Program");
            choice = read!();
            
            if choice == 8{
                return;
            }
        }
        //Call the correct function based on user's choice
        match choice {
            1 => UI::display_menu(self),
            2 => UI::display_hired(self),
            3 => UI::hire_emp(self),
            4 => UI::fire_emp(self),
            5 => UI::order_item(self),
            6 => UI::change_item_price(self),
            7 => UI::sim_day(self),
            _ => println!("Error"),
        }
    }

    //Display functions
    pub fn display_menu(&mut self) {
        self.sim.display_menu();
        UI::home_page(self);
    }
    
    pub fn display_hired(&mut self) {
        self.sim.display_hired();
        UI::home_page(self);
    }
   
    pub fn display_pot(&mut self){
        self.sim.display_pot();
        UI::home_page(self);
    }
    
    //Used to add an employee from the potential employee list to hired list
    pub fn hire_emp(&mut self) {
        
        let mut emp_select : i64 = -1;
        //Display potential employees and let user choose an employee to hire
        while emp_select < 1 || emp_select > self.sim.pot_len() {
           self.sim.display_pot();
            println!("Enter 0 to return to home page");
            println!("Choose employee to hire : ");
            emp_select = read!();
            if emp_select == 0 {
                UI::home_page(self);
            }
        }
        //higher selected employee
        self.sim.hire_empl((emp_select - 1) as usize);
        self.sim.display_hired();
        UI::home_page(self);
    }
    
    //Used to remove an employee from the hired list
    pub fn fire_emp(&mut self) {
        let mut emp_select : i64 = -1;
        //Display hired employees and let user choose one to fire
        while emp_select < 1 || emp_select > self.sim.hired_len() {
           self.sim.display_hired();
            println!("Enter 0 to return to home page");
           println!("Choose employee to fire : ");
            emp_select = read!();
            if emp_select == 0 {
                UI::home_page( self);
            }
        }
        //Remove selected employee from the hired list
        self.sim.fire_empl((emp_select - 1) as usize);
        self.sim.display_hired();

         UI::home_page(self);
    }

    pub fn order_item(&mut self) {
        let mut name: String = "".to_owned();
        self.sim.display_inv();
        println!("Enter the number of the menu item to order : ");
        println!("[1] Burger \n[2] Fries \n[3] Soda");
        let mut item_select: i32 = read!();

        while item_select < 1 || item_select > 3 {
            println!("Enter the number of the menu item to order: ");
            println!("[1] Burger \n[2] Fries \n[3] Soda");
            item_select = read!();
        }

        if (item_select == 1) {
            name = "Burger".to_owned();
        }
        if (item_select == 2) {
            name = "Fries".to_owned();
        }
        if (item_select == 3) {
            name = "Soda".to_owned();
        }
        println!("Current {} quality: {} \nCurrent inventory of {}: {}",name.clone(), self.sim.get_item_quality(name.clone()), name.clone(), self.sim.get_inv(name.clone()));

        println!(
            "Would you like to change the quality of {}?\n y for yes, any other key for no: ",
            name
        );

        let change_quality: String = read!();
        if change_quality == "y".to_owned() {
            UI::change_item_quality(self);
        }

        println!("Enter amount of {} to order", name.clone());
        let inc_amount: i64 = read!();

        self.sim.order_inv(name.clone(), inc_amount);
        println!("Current {} quality: {}
                 \nCurrent inventory of {}: {}",name.clone(), self.sim.get_item_quality(name.clone()), name.clone(), self.sim.get_inv(name.clone()));
        UI::home_page(self);
    }
    
    //Used to change the quality of an item
    pub fn change_item_quality(&mut self) {
            let mut new_quality: i64 = -1;
            let mut name : String = "".to_owned();
            //Get new quality (Must be 
            while new_quality < 1 || new_quality > 3 {
                println!("Select quality ");
                println!("[1] Low\n[2] Medium\n[3] High");
                new_quality = read!();
            }

            if (new_quality == 1) {
                name = "Burger".to_owned();
            }
            if (new_quality == 2) {
                name = "Fries".to_owned();
            }
            if (new_quality == 3) {
                name = "Soda".to_owned();
            }

            self.sim.set_item_quality(name.clone(), new_quality);
    }
    
    //Changes the price of a menu item
    pub fn change_item_price(&mut self) {
            let mut item: i64 = -1;
            let mut name: String = " ".to_owned();
           //Get an item
            while item < 1 || item > 3 {
                println!("Select Menu Item to change ");
                item = read!();
            }

            if (item == 1) {
                name = "Burger".to_owned();
            }
            if (item == 2) {
                name = "Fries".to_owned();
            }
            if (item == 3) {
                name = "Soda".to_owned();
            }

            println!("Enter the new price of {}:",name);
            let new_price = read!();
            self.sim.set_item_price(name.clone(), new_price); 
            UI::home_page(self);
    }

    //Simulates a single day for the restaurant.
    //Creates customers to serve
    //Updates revenue based on the amount sold and the cost of labor
    pub fn sim_day(&mut self) {
        self.sim.sim_day();
        UI::home_page(self);
    }
}
