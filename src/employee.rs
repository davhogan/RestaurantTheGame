extern crate rand;
extern crate lazy_static;

use rand::thread_rng;
use std::sync::Mutex;

const MINWAGE: f64 = 7.25;
static mut ID : i64 = 0;

//lazy_static! {
  // static ref ID : Mutex i64 = Mutex :: 0;
//}

//This is the employee
struct Employee{
   // name : String, //Employee name
    id : i64,
    age : i64, //Employee age
    wage : f64, //Employee wage
    max_hrs: f64, //Maximum hours an employee can work
    hrs_worked: f64, //Hours employee has worked
    rating : i64, //Rating of employee ranges from 1-10
    position: i64, //Name of position
}

impl Employee {
    //Default employee constructor
    fn new(id:i64, age:i64, wage:f64, position: String) -> Employee{
        let mut max_hours = 40.0;

        if age < 18 {
            max_hours = 20.0;
        }

        Employee{id, age, wage, max_hrs, hrs_worked : 0.0, rating : 5, position}
    }

    fn setWage(&mut self, new_wage:f64){
        self.wage = new_wage;
    }

    fn setRating(&mut self, new_rating:i64){
        self.rating = new_rating;
    }

    //Generates a random employee
    fn randEmployee()-> Employee{
        let mut id: ID;
        let age = rng.gen_range(16,75);
        let rating = rng.gen_range(1,10);
        let mut wage = 7.25;

        if rating > 5 {
            wage += (rating - 5);
        }

        let mut max_hrs = 40.0;

        if age < 18 {
            max_hrs = 20.0;
        }

       let position = rng.gen_range(0,2);

        Employee {id, age, wage, hrs_worked : 0.0, max_hrs, rating, position}
    }
}