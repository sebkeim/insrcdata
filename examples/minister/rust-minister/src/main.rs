use crate::insrcdata::{Countries, Country, Minister, Ministers};

#[allow(dead_code)]
#[allow(unused_variables)]
mod insrcdata;
mod opendataset;

// overview of insrcdata

fn main() {
    let g_brown: &Minister = Ministers::GordonBrown.into();

    // access it's attributes
    println!("{} was born in {}.", g_brown.name(), g_brown.birth());

    // navigate between linked table
    println!("He was prime minister of {}.", g_brown.country().name());

    // perform indexed searches
    for minister in Minister::birth_range(1900, 1960) {
        println!("{}", minister.name());
    }

    // perform reverse lookup between tables
    for minister in <&Country>::from(Countries::Uk).ministers() {
        println!("{}", minister.name());
    }

    // sample of implementation for open dataset model
    opendataset::inapp::sample();
    opendataset::inlib::sample();
}
