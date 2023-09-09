// make available code generated from build.rs
#[allow(dead_code)]
#[allow(unused_variables)]
mod insrcdata {
    include!(concat!(env!("OUT_DIR"), "/insrcdata.rs"));
}

// minimalistic sample that print the paradigmatic sentence in the console
// note the use of the method ::array() to access all the row of the table
// this method is activated by the corresponding flag in the config file insrcdata.toml

fn main() {
    for fic in insrcdata::HelloWorld::array() {
        println!("{}", fic.sentence());
    }
}
