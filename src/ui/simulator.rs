// Copyright © 2019 David Hogan
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

use rand::Rng;
mod restaurant;
use restaurant::Restaurant;

pub struct Simulator {
    day : i64,
    restaurant :  Restaurant,
}

impl Simulator {
    pub fn new() -> Simulator {
    println!("Enter name of restaurant: ");
    let name : String = read!("{}\n");

    let restaurant = Restaurant::new(name);

    Simulator{day : 0, restaurant}
    }
   
    //Simulates a day in the restaurant
    //Call the restaurant function to serve the customers
    //Updates the revenue based on items sold and total cost of labor 
    pub fn sim_day(&mut self) {
        self.day += 1;
        let revenue : f64 = self.restaurant.get_revenue();
        let burg_inv : i64 = self.restaurant.get_inv("Burger".to_owned());
        let fries_inv : i64 = self.restaurant.get_inv("Fries".to_owned());
        let soda_inv : i64 = self.restaurant.get_inv("Soda".to_owned());
    
        let cust_served = self.restaurant.generate_customers();                  
        self.restaurant.serve_customers();
        self.restaurant.reduce_profit(self.restaurant.calc_empl_day_cost());

        println!("Customers servered : {}", cust_served);
        println!("Burgers Sold: {}", burg_inv - self.restaurant.get_inv("Burger".to_owned()));
        println!("Orders of Fries Sold: {}", fries_inv - self.restaurant.get_inv("Fries".to_owned()));
        println!("Sodas Sold: {}", soda_inv - self.restaurant.get_inv("Soda".to_owned()));
        println!("Daily profit: ${}", self.restaurant.get_revenue() - revenue);
    }
   
    //Getters 
    pub fn get_day(&mut self) -> i64 {
        self.day
    }

    pub fn get_inv(&mut self, name : String) -> i64{
        self.restaurant.get_inv(name.clone())
    }
    pub fn get_name(&mut self) -> String {
        self.restaurant.get_name()
    }
    
    pub fn hired_len(&mut self) -> i64 {
         self.restaurant.get_hired_empls().len() as i64
    }

    pub fn pot_len(&mut self) -> i64 {
        self.restaurant.get_pot_empls().len() as i64
    }

    pub fn get_revenue(&mut self) -> f64 {
        self.restaurant.get_revenue()
    }

    pub fn get_item_quality(&mut self, name: String) -> i64 {
        self.restaurant.get_item_quality(name.clone())
    }

    //Setters
    pub fn set_item_quality(&mut self, name: String, new_quality : i64) {
        self.restaurant.set_item_quality(name, new_quality);
    }

    pub fn set_item_price(&mut self, name : String, new_price : f64){
        self.restaurant.set_item_price(name,new_price);
    }


    //Hire an employee based on position given in list
    pub fn hire_empl(&mut self, position : usize) {
        let new_emp = self.restaurant.get_pot_empl(position);
        self.restaurant.hire_emp(new_emp);
    }

    //Fire an employee based on position given in list
    pub fn fire_empl(&mut self, position : usize) {
        let fired_emp = self.restaurant.get_hired_empl(position);
        self.restaurant.fire_emp(fired_emp.get_id());
    }
    
    //Helper function to calaculate how much it costs 
    //for the restaurant to order one item based on the quality
    fn calc_item_price(name: String, quality: i64) -> f64 {
        if name == "Burger".to_owned() {
            if quality == 1 {
                return 2.50;
            }
            else if quality == 2 {
                return 4.00;
            }
            else {
                return 5.50;
            }
        }
        else if name == "Fries".to_owned() {
            if quality == 1 {
                return 1.00;
            }
            else if quality == 2 {
                return 1.50;
            }
            else {
                return 2.00;
            }
        }
        //Soda
        else {
            if quality == 1 {
                return 0.25
            }
            else if quality == 2 {
                return 0.37
            }
            else {
                return 0.50
            }
        }
       0.0
    }

    //Increases inventory of chosen item
    //Reduces revenue by the amount of items ordered times cost for the good
    pub fn order_inv(&mut self, name : String, inc_amount : i64) {
        let quality = self.restaurant.get_item_quality(name.clone());
        let item_price = Simulator::calc_item_price(name.clone(),quality);
        self.restaurant.reduce_profit(inc_amount as f64 * item_price);
        self.restaurant.inc_inv(name.clone(),inc_amount)
    }

    //Displays  
    pub fn display_hired(&self) {
        self.restaurant.display_hired();
        println!();
    }

    pub fn display_pot(&self) {
        self.restaurant.display_pot();
        println!();
    }

    pub fn display_menu(&self) {
        self.restaurant.display_menu();
        println!();
    }

    pub fn display_inv(&mut self) {
        self.restaurant.display_inv();   
        println!();
    }


}
