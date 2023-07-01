use crate::insrcdata::{Countries, Minister, Ministers};

mod insrcdata;

// overview of insrcdata

fn main() {
    // get individual elements
    let g_brown = Ministers::GordonBrown;

    // access it's attributes
    println!("{} was born in {}", g_brown.name(), g_brown.birth());

    // navigate between linked table
    println!("he was minister of {}", g_brown.country().name());

    // perform indexed searches
    for minister in Minister::birth_range(1900, 1960) {
        println!("{}", minister.name());
    }

    // perform reverse lookup between tables
    for minister in Countries::Gb.ministers() {
        println!("{}", minister.name());
    }
}
