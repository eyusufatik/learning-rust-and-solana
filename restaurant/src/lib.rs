mod front_of_house;
mod back_of_house;


mod costumer {
    use crate::front_of_house::hosting;
    use crate::back_of_house::{Breakfast, Appetizer};
    
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    
        hosting::add_to_waitlist();
    
        let mut meal = Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
    
        let order1 = Appetizer::Soup;
        let order2 = Appetizer::Salad;
    }
}

fn deliver_order() {}
