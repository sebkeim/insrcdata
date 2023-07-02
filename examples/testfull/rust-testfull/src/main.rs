mod colobject;
mod fictolabel;
mod innerjoin;
mod insrcdata;
mod strencoding;

// Check for various functions that not have yet a specific tutorial

fn main() {
    // the join column reference a record in the same table
    innerjoin::test_innerjoin();

    // retrieve label from fic record reference
    fictolabel::test_fictolabel();

    // check string comparison for various encoded unicode strings
    strencoding::test_strencoding();

    // object type column : reference to native objects
    colobject::test_colobject();
}

// TODO : show how to use a second insrcdata database in the same project
