mod fictolabel;
mod insrcdata;
mod innerjoin;

// Check for various functions that not have yet a specific tutorial

fn main() {
    // the join column reference a record in the same table
    innerjoin::test_innerjoin();

    //  retrieve label from fic record reference
    fictolabel::test_fictolabel();
}

// TODO : show how to use a second insrcdata database in the same project
