mod adhoc;
mod insrcdata;
mod optjoin;

// there is curently no builtin support for optional values
// this sample show available strategies to handle optional values

fn main() {
    optjoin::sample();
    adhoc::sample();
}
