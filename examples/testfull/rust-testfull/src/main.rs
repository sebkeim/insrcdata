extern crate core;

mod colobject;
mod fictolabel;
mod innerjoin;
mod insrcdata;
mod strencoding;

// Check for various functions that not have yet a specific tutorial

fn main() {
    // the join column reference a record in the same table
    innerjoin::test_innerjoin();

    // bool values
    innerjoin::test_bool();

    // float values
    innerjoin::test_float();

    // retrieve label from fic record reference
    fictolabel::test_fictolabel();

    // check string comparison for various encoded unicode strings
    strencoding::test_strencoding();

    // object type column : reference to native objects
    colobject::test_colobject();

    // variant column
    innerjoin::test_variant_non_optional();

    // variant with unmatched rows
    innerjoin::test_variant_optional()
}

// TODO : show how to use a second insrcdata database in the same project
