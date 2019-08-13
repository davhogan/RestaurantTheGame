// Copyright © 2019 David Hogan
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

use rand::Rng;
use std::cell::Cell;
mod names;
use names::{Generator, Name};

const MINWAGE: f64 = 7.25;

const COOK: i64 = 0;
const SERVER: i64 = 1;
const WASHER: i64 = 2;
const BUSSER: i64 = 3;
const HOST: i64 = 4;

//This is the employee
#[derive(Clone)]
pub struct Employee {
    name: String,
    id: i64,
    wage: f64,
    //Rating of employee ranges from 1-10
    rating: i64,
    //Name of position
    posit: i64,
}

impl Employee {
    //Default employee constructor
    //Used to fill the hired employee list in a Restaurant upon creation of said Restaurant.
    pub fn default(posit: i64, id: i64) -> Employee {
        let mut rng = rand::thread_rng();
        let mut generator = Generator::with_naming(Name::Plain);
        let name = generator.next().unwrap();
        let rating = 5;
        let mut wage = MINWAGE;

        Employee {
            name,
            id,
            wage,
            rating,
            posit,
        }
    }

    //Generates a random employee
    //Used to fill potential employees list for a Restaurant
    pub fn rand_empl() -> Employee {
        let mut rng = rand::thread_rng();
        let mut generator = Generator::with_naming(Name::Plain);
        let name = generator.next().unwrap();
        let mut id = rng.gen_range(1, 99999);
        let rating = rng.gen_range(1, 11);
        let mut wage = MINWAGE;
        let posit = rng.gen_range(0, 5);

        if rating > 5 {
            wage += rating as f64 - 5.0;
        }

        Employee {
            name,
            id,
            wage,
            rating,
            posit,
        }
    }

    pub fn get_name(self) -> String {
        self.name
    }

    pub fn get_wage(&self) -> f64 {
        self.wage
    }

    pub fn get_posit(&self) -> i64 {
        self.posit
    }

    pub fn get_posit_string(&self) -> String {
        if self.posit == 0 {
            return "Cook".to_owned();
        } else if self.posit == 1 {
            return "Server".to_owned();
        } else if self.posit == 2 {
            return "Washer".to_owned();
        } else if self.posit == 3 {
            return "Busser".to_owned();
        } else {
            "Host".to_owned()
        }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_rating(&self) -> i64 {
        self.rating
    }

    pub fn set_id(&mut self, new_id: i64) {
        self.id = new_id;
    }
}

#[test]
fn employee_test() {
    let emp_test = Employee::new();

    assert!(emp_test.age > 15 && emp_test.age < 76);
    assert!(emp_test.wage >= MINWAGE);
    assert!(emp_test.max_hrs > 19.0 && emp_test.max_hrs < 41.0);
    assert_eq!(0.0, emp_test.hrs_worked);
    assert!(emp_test.rating > 0 && emp_test.rating < 11);
    assert!(emp_test.position >= 0 && emp_test.position <= 2);
}

#[test]
fn set_wage_test() {
    let mut emp_test = Employee::new();
    Employee::set_wage(&mut emp_test, 12.34);
    assert_eq!(12.34, emp_test.wage);
}

#[test]
fn set_rating_test() {
    let mut emp_test = Employee::new();
    Employee::set_rating(&mut emp_test, 8);
    assert_eq!(8, emp_test.rating);
}

#[test]
fn set_rating_test_over_ten() {
    let mut emp_test = Employee::new();
    Employee::set_rating(&mut emp_test, 11);
    assert_eq!(10, emp_test.rating);
}

#[test]
fn set_rating_test_under_one() {
    let mut emp_test = Employee::new();
    Employee::set_rating(&mut emp_test, 0);
    assert_eq!(1, emp_test.rating);
}
