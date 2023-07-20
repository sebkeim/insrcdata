use crate::insrcdata;
//
// ad-hoc strategy use placeholder to represent 'no value'
//

// placeholder values that represent 'no value'
// we must be certain that theses values will never be needed for real data
const SCORE_EMPTY: f32 = -1.0;
const COUNT_EMPTY: u16 = 0;

// implement optional getter from methods generated by insrcdata
pub trait Adhoc {
    // getter for optional f32 value
    fn score(&self) -> Option<f32> {
        match self.score_data() {
            // avoid warning: floating-point types cannot be used in patterns
            v if v == SCORE_EMPTY => None,
            v => Some(v),
        }
    }

    // getter for optional u16 value
    fn count(&self) -> Option<u16> {
        match self.count_data() {
            COUNT_EMPTY => None,
            v => Some(v),
        }
    }

    #[doc(hidden)]
    // insrcdata methods
    fn score_data(&self) -> f32;
    fn count_data(&self) -> u16;
}

// print the content of a row
fn print(row: &dyn Adhoc, name: &str) {
    print!("{}", name);
    if let Some(score) = row.score() {
        print!(" score={}", score);
    }
    if let Some(count) = row.count() {
        print!(" count={}", count);
    }
    println!();
}

//  ad-hoc strategy usage sample
pub fn sample() {
    println!("    Adhoc strategy");
    print(&*insrcdata::Adhocs::Filled, "Filled");
    print(&*insrcdata::Adhocs::Empty, "Empty");
    println!();
}
