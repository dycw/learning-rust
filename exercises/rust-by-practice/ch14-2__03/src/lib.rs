mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}

        fn complain() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();

        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}
