mod insrcdata;

// minimalistic sample that print the paradigmatic sentence in the console
// note the use of the method ::array() to access all the row of the table
// this method is activated by the corresponding flag in the config file insrcdata.toml

fn main() {
    for fic in insrcdata::HelloWorld::array() {
        println!("{}", fic.sentence());
    }
}
