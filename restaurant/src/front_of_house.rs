
pub mod hosting {
    // note that both gotta be pub
    pub fn add_to_waitlist() {
        println!("adding to waitlist...");
    }

    fn seat_at_table() {}
}

pub mod serving {
    fn take_order() {}

    pub fn serve_order() {
        // super::deliver_order(); // FIXME
    }

    fn take_payment() {}
}
