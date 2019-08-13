mod employee;
use employee::Employee;
mod menu_item;
use menu_item::MenuItem;
use rand::Rng;
use std::cmp;

const COOK: i64 = 0;
const SERVER: i64 = 1;
const WASHER: i64 = 2;
const BUSSER: i64 = 3;
const HOST: i64 = 4;
const MIN_CUST: i64 = 5;
const MAX_CUST: i64 = 25;

#[derive(Clone)]
struct Customer {
    cash: f64,
    likes_fries: bool,
    likes_soda: bool,
}

impl Customer {
    pub fn new() -> Customer {
        let mut rng = rand::thread_rng();
        let mut likes_fries = false;
        let mut likes_soda = false;

        if rng.gen_range(0, 2) == 1 {
            likes_fries = true;
        }

        if rng.gen_range(0, 2) == 1 {
            likes_soda = true;
        }

        Customer {
            cash: rng.gen_range(10.0, 100.0),
            likes_fries,
            likes_soda,
        }
    }

    pub fn get_cash(&self) -> f64 {
        self.cash
    }

    pub fn get_likes_fries(&self) -> bool {
        self.likes_fries
    }

    pub fn get_likes_soda(&self) -> bool {
        self.likes_soda
    }

    pub fn reduce_cash(&mut self, amount: f64) {
        self.cash -= amount;
    }
}

#[derive(Clone)]
pub struct Restaurant {
    name: String,
    revenue: f64,
    id: i64,
    menu: Vec<MenuItem>,
    hired_empls: Vec<Employee>,
    pot_empls: Vec<Employee>,
    customers: Vec<Customer>,
    empl_cost: f64,
    goods_cost: f64,
}

impl Restaurant {
    pub fn new(name: String) -> Restaurant {
        let mut menu: Vec<MenuItem> = Vec::new();
        let mut hired_empls: Vec<Employee> = Vec::new();
        let mut pot_empls: Vec<Employee> = Vec::new();
        let mut customers: Vec<Customer> = Vec::new();
        let mut empl_cost = 0.0;
        let mut goods_cost = 0.0;
        let revenue = 1000.00;
        let mut id = 0;

        menu.push(MenuItem::new("Burger".to_owned(), 5.00, 1));
        menu.push(MenuItem::new("Fries".to_owned(), 2.00, 1));
        menu.push(MenuItem::new("Soda".to_owned(), 1.00, 1));

        hired_empls.push(Employee::default(COOK, id + 1));
        id += 1;
        hired_empls.push(Employee::default(SERVER, id + 1));
        id += 1;
        hired_empls.push(Employee::default(WASHER, id + 1));
        id += 1;

        for _ in 0..10 {
            pot_empls.push(Employee::rand_empl());
        }

        for empl in &hired_empls {
            empl_cost += empl.get_wage() * 8.0;
        }

        Restaurant {
            name,
            revenue,
            id,
            menu,
            hired_empls,
            pot_empls,
            customers,
            empl_cost,
            goods_cost,
        }
    }

    //Getters
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_revenue(&self) -> f64 {
        self.revenue
    }

    pub fn get_empl_cost(&self) -> f64 {
        self.empl_cost
    }

    pub fn get_goods_cost(&self) -> f64 {
        self.goods_cost
    }

    pub fn get_menu(self) -> Vec<MenuItem> {
        self.menu
    }

    pub fn get_hired_empls(&mut self) -> Vec<Employee> {
        self.hired_empls.clone()
    }

    pub fn get_pot_empls(&mut self) -> Vec<Employee> {
        self.pot_empls.clone()
    }

    pub fn get_customers(self) -> Vec<Customer> {
        self.customers
    }

    pub fn get_hired_empl(&self, position: usize) -> &Employee {
        &self.hired_empls[position]
    }

    pub fn get_pot_empl(&self, position: usize) -> Employee {
        self.pot_empls[position].clone()
    }

    //Setters
    pub fn set_goods_cost(&mut self, new_cost: f64) {
        self.goods_cost = new_cost;
    }

    pub fn set_revenue(&mut self, new_rev: f64) {
        self.revenue = new_rev;
    }

    pub fn set_customers(&mut self, new_customers: Vec<Customer>) {
        self.customers = new_customers;
    }

    pub fn set_item_quality(&mut self, name: String, new_quality: i64) {
        for mut item in &mut self.menu {
            if name == item.get_name() {
                item.set_quality(new_quality);
                return;
            }
        }
    }

    pub fn set_item_price(&mut self, name: String, new_price: f64) {
        for mut item in &mut self.menu {
            if name == item.get_name() {
                item.set_price(new_price);
                return;
            }
        }
    }
    //Hire
    pub fn hire_emp(&mut self, mut new_emp: Employee) {
        self.pot_empls.retain(|x| x.get_id() != new_emp.get_id());
        self.id +=1;
        new_emp.set_id(self.id);
        self.hired_empls.push(new_emp);
    }

    //Fire
    pub fn fire_emp(&mut self, id: i64) {
        self.hired_empls.retain(|x| x.get_id() != id);
    }

    //Deduct
    pub fn reduce_profit(&mut self, cost: f64) {
        self.revenue -= cost;
    }

    //Increase
    pub fn inc_profit(&mut self, profit: f64) {
        self.revenue += profit;
    }

    //Increment Inventory for given item and increment by the given amount
    pub fn inc_inv(&mut self, name: String, inc_amount: i64) {
        for mut item in &mut self.menu {
            if name == item.get_name() {
                item.inc_inv(inc_amount);
                return;
            }
        }
    }

    //Decrement Inventory for given item and decrement by the given amount
    pub fn reduce_inv(&mut self, name: String, dec_amount: i64) {
        for mut item in &mut self.menu {
            if name == item.get_name() {
                item.dec_inv(dec_amount);
            }
        }
    }

    pub fn get_price(self, name: String) -> f64 {
        let mut price: f64 = 0.0;

        for item in self.menu {
            if name == item.get_name() {
                price = item.get_price()
            }
        }

        price
    }

    pub fn get_item_quality(&mut self, name: String) -> i64 {
        let mut quality: i64 = 0;

        for item in &mut self.menu {
            if name == item.get_name() {
                quality = item.get_quality();
            }
        }
        quality
    }

    pub fn get_inv(&mut self, name: String) -> i64 {
        let mut inv: i64 = 0;

        for item in &mut self.menu {
            if name == item.get_name() {
                inv = item.get_inv();
            }
        }
        inv
    }

    pub fn set_inv_quality(&mut self, name: String, new_qual: i64) {
        for mut item in &mut self.menu {
            if name == item.get_name() {
                item.set_quality(new_qual);
            }
        }
    }

    pub fn calc_cust_serv(&self) -> i64 {
        let mut tot_rating = 0;

        for emp in &self.hired_empls {
            tot_rating += emp.get_rating();
        }

        tot_rating
    }

    pub fn num_cust_helper(quality: i64) -> i64 {
        //Poor Quality
        if quality <= 3 {
            return 1;
        }
        //Medium Quality
        if quality > 3 && quality <= 5 {
            return 10;
        }
        //Higher Quality
        if quality > 5 && quality <= 8 {
            return 20;
        }
        //Highest in all qualities
        25
    }
    
    //Used to calculate the modifier for the amount of customers
    //Uses the total quality to get a range from the helper function
    pub fn num_cust_mod(&mut self) -> i64 {
        let mut tot_quality = 0;

        for item in &self.menu {
            tot_quality += item.get_quality();
        }

        Restaurant::num_cust_helper(tot_quality)
    }

    //Generates the list of customers for a restaurant.
    //Number is determined based on quality of the items.
    //Or based on ratings of hired employees
    pub fn generate_customers(&mut self) -> i64 {
        let mut new_customers: Vec<Customer> = Vec::new();
        let mut rng = rand::thread_rng();
        let num_cust = rng.gen_range(
            MIN_CUST + self.num_cust_mod(),
            MAX_CUST + self.num_cust_mod(),
        );
        let min = cmp::min(self.calc_cust_serv(), num_cust);
        for _ in 0..min {
            new_customers.push(Customer::new());
        }

        self.customers = new_customers;
        self.customers.len() as i64
    }
   
    //Goes through the list of customers
    //Every customer orders a burger if they have enough money
    //If the customer likes soda or fries they will order one if they have enough money
    //No orders will occur if the inventory for the item is zero
    pub fn serve_customers(&mut self) {
        let burg_price = self.clone().get_price("Burger".to_owned());
        let fries_price = self.clone().get_price("Fries".to_owned());
        let soda_price = self.clone().get_price("Soda".to_owned());

        for customer in self.clone().customers {
            if customer.get_cash() >= burg_price && self.get_inv("Burger".to_owned()) > 0 {
                self.reduce_inv("Burger".to_owned(), 1);
                self.inc_profit(burg_price);
            }
            if customer.get_likes_fries() {
                if customer.get_cash() >= fries_price
                    && self.get_inv("Fries".to_owned()) > 0
                {
                    self.reduce_inv("Fries".to_owned(), 1);
                    self.inc_profit(fries_price);
                }
            }
            if customer.get_likes_soda() {
                if customer.get_cash() >= soda_price && self.get_inv("Soda".to_owned()) > 0
                {
                    self.reduce_inv("Soda".to_owned(), 1);
                    self.inc_profit(soda_price);
                }
            }
        }
    }

    //Calculate costs
    //Goes through list of hired empls 
    //Calculates cost of labor for the day for each employee
    pub fn calc_empl_day_cost(&self) -> f64 {
        let mut empl_cost = 0.0;

        for empl in &self.hired_empls {
            empl_cost += empl.get_wage() * 8.0;
        }

        empl_cost
    }


    //Displays
    pub fn display_hired(&self) {
        let mut i = 1;
        println!("\tName\tID\tWage\tPostition\tRating");
        for empl in &self.hired_empls {
            println!(
                "[{}]\t{}\t{}\t{}\t{}\t\t{}",
                i,
                empl.clone().get_name(),
                empl.get_id(),
                empl.get_wage(),
                empl.get_posit_string(),
                empl.get_rating()
            );
            i += 1;
        }
    }

    pub fn display_pot(&self) {
        let mut i = 1;
        println!("\tName\tID\tWage\tPostition\tRating");
        for empl in &self.pot_empls {
            println!(
                "[{}]\t{}\t{}\t{}\t{}\t\t{}",
                i,
                empl.clone().get_name(),
                empl.get_id(),
                empl.get_wage(),
                empl.get_posit_string(),
                empl.get_rating()
            );
            i += 1;
        }
    }

    pub fn display_menu(&self) {
        let mut i = 1;
        println!("\tItem\tPrice\tQuality");
        for item in &self.menu {
            println!(
                "[{}]\t{}\t${}\t{}",
                i,
                item.get_name(),
                item.get_price(),
                item.get_quality()
            );
            i += 1;
        }
    }

    pub fn display_inv(&mut self) {
        println!(
            "Number of burgers available to sell: {} ",
            self.get_inv("Burger".to_owned())
        );
        println!(
            "Number of orders fries available to sell: {} ",
            self.get_inv("Fries".to_owned())
        );
        println!(
            "Number of sodas available to sell: {} ",
            self.get_inv("Soda".to_owned())
        );
    }
}
