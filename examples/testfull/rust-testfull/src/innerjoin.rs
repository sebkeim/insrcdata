use crate::insrcdata as db;

// Inner join is when your join column reference a record in the same table
pub fn test_innerjoin() {
    let marie = db::Persons::Marie;
    let pierre = db::Persons::Pierre;
    let irene = db::Persons::Irene;
    let frederic = db::Persons::Frederic;

    assert!(marie.name() == "Marie Curie");
    assert!(pierre.name() == "Pierre Curie");
    assert!(irene.name() == "Irène Joliot-Curie");
    assert!(frederic.name() == "Frédéric Joliot-Curie");

    // inner join with 1-1 cardinality
    assert!(pierre == marie.spouse());
    assert!(marie == pierre.spouse());
    assert!(frederic == irene.spouse());
    assert!(irene == frederic.spouse());

    // inner join with 0-1 cardinality
    assert!(marie.mother().is_none());
    assert!(marie.father().is_none());
    assert!(pierre.mother().is_none());
    assert!(pierre.father().is_none());
    assert!(frederic.mother().is_none());
    assert!(frederic.father().is_none());
    assert!(marie == irene.mother().expect("Irene has mother"));
    assert!(pierre == irene.father().expect("Irene has father"));
}

pub fn test_bool() {
    let marie = db::Persons::Marie;
    let pierre = db::Persons::Pierre;

    assert!(marie.woman());
    assert!(!pierre.woman());

    /*
    insrcdata generated code violate clippy::bool_comparison check

    // test iterator
    let womens: Vec<String> = db::Person::woman_range(true, true)
        .map(|n| n.name().to_string())
        .collect();
    assert!(womens == vec!["Marie Curie", "Irène Joliot-Curie"]);
    */
}

pub fn test_float() {
    let marie = db::Persons::Marie;
    let pierre = db::Persons::Pierre;

    assert!(marie.score() == 1.0);
    assert!(pierre.score() == 2.1);

    // test closed range iterator
    // the iterator is stable : table order is preserved for equal values
    // we get all lower bound values
    let middle: Vec<String> = db::Person::score_range(2.1, 3.2)
        .map(|n| n.name().to_string())
        .collect();
    assert!(
        middle
            == vec![
                "Pierre Curie",
                "Frédéric Joliot-Curie",
                "Irène Joliot-Curie",
            ]
    );

    // we get all upper bound values
    let middle: Vec<String> = db::Person::score_range(1.0, 2.1)
        .map(|n| n.name().to_string())
        .collect();
    assert!(middle == vec!["Marie Curie", "Pierre Curie", "Frédéric Joliot-Curie",]);
}
