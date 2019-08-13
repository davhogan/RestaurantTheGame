// Copyright © 2019 David Hogan
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

use std::borrow::Borrow;

#[derive(Clone)]
pub struct MenuItem {
    name: String,
    price: f64,
    quality: i64,
    inv: i64,
}

impl MenuItem {
    pub fn new(name: String, price: f64, quality: i64) -> MenuItem {
        MenuItem {
            name,
            price,
            quality,
            inv: 100,
        }
    }

    //Getters
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn get_quality(&self) -> i64 {
        self.quality
    }

    pub fn get_inv(&self) -> i64 {
        self.inv
    }

    //Setters
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn set_quality(&mut self, new_quality: i64) {
        self.quality = new_quality;
    }

    pub fn set_price(&mut self, new_price: f64) {
        self.price = new_price;
    }

    pub fn set_inv(&mut self, new_inv: i64) {
        self.inv = new_inv;
    }

    //Increases Inventory by the given amount.
    //Allows the user to order more inventory for a specific item.
    pub fn inc_inv(&mut self, inc_amount: i64) {
        self.inv += inc_amount;
    }

    //Decrease Inventory
    //Used when a customer orders an item off the menu.
    pub fn dec_inv(&mut self, dec_amount: i64) {
        if self.inv <= 0 {
            return;
        }
        self.inv -= dec_amount;
    }
}

#[test]
fn inc_inv_test_under_one() {
    let mut emp_test = Employee::new();
    Employee::set_rating(&mut emp_test, 0);
    assert_eq!(1, emp_test.rating);
}