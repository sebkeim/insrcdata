//
// ad-hoc strategy use placeholder to represent 'no value'
//

use crate::{insrcdata, Optional};

// placeholder values that represent 'no value'
// we must be certain that theses values will never be needed for real data
const SCORE_EMPTY: f32 = -1.0;
const COUNT_EMPTY: u16 = 0;

// implement optional getter from methods generated by insrcdata
impl Optional for insrcdata::Adhoc {
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
}

//  ad-hoc strategy usage sample
pub fn sample() {
    println!("    Adhoc strategy");
    insrcdata::Adhocs::Filled.print("Filled");
    insrcdata::Adhocs::Empty.print("Empty");
    println!();
}
