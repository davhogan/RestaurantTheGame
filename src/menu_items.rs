mod menu_items{
    use std::borrow::Borrow;

    pub struct MenuItem {
        name : String,
        price : f64,
        quality : i64,
    }

    impl MenuItem{
        fn new(name:String, price:f64, quality: i64) -> MenuItem {
            MenuItem {name, price, quality}
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

        //Setters
        pub fn set_name(&mut self, new_name: String)  {
            self.name = new_name;
        }

        pub fn set_quality(&mut self, new_quality : i64) {
            self.quality = new_quality;
        }

        pub fn set_price(&mut self, new_price: f64)  {
            self.price = new_price;
        }
    }
}