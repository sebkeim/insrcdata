use crate::insrcdata as db;

// Pattern matching : retrieve label from fic record reference

fn nobel_year(x: &db::Person) -> u16 {
    match x {
        x if x == db::Persons::Marie => 1911, // also in 1903
        x if x == db::Persons::Pierre => 1903,
        x if x == db::Persons::Irene => 1935,
        x if x == db::Persons::Frederic => 1935,
        _ => 0,
    }

    // In rust there is [no builtin way to retrieve enum from it's numeric value](https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html)
    //  you have to use  [double match](https://stackoverflow.com/questions/68677383/match-integers-against-enum-cases-without-a-double-match-statement-in-rust)
    //  pattern matching will be quite inefficient if there is many values to check
    //
    //  The recommended way to overcome this problem is to use the ``as_label` flag  :
    //  to embed enum reference in table see labels sample for usage informations
}

pub fn test_fictolabel() {
    let marie: &db::Person = db::Persons::Marie.into();
    let irene: &db::Person = db::Persons::Irene.into();

    assert!(nobel_year(marie.spouse()) == 1903);
    assert!(nobel_year(irene) == 1935);
}
