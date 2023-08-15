mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let _order1 = back_of_house::Appetizer::Soup; // help: if this is intentional, prefix it with an underscore: `_order1`
    let _order2 = back_of_house::Appetizer::Salad; // help: if this is intentional, prefix it with an underscore: `_order2`
}
