fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn relative_super() {
    // Absolute path
    crate::modules::visibility::paths_relative_with_super::back_of_house::fix_incorrect_order();

    // Relative path
    super::paths_relative_with_super::back_of_house::fix_incorrect_order();
}
