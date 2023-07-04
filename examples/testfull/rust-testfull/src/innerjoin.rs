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
    assert!(std::ptr::eq(marie.spouse(), &*pierre));
    assert!(std::ptr::eq(pierre.spouse(), &*marie));
    assert!(std::ptr::eq(irene.spouse(), &*frederic));
    assert!(std::ptr::eq(frederic.spouse(), &*irene));

    // inner join with 0-1 cardinality
    assert!(marie.mother().is_none());
    assert!(marie.father().is_none());
    assert!(pierre.mother().is_none());
    assert!(pierre.father().is_none());
    assert!(frederic.mother().is_none());
    assert!(frederic.father().is_none());
    assert!(std::ptr::eq(
        irene.mother().expect("Irene has mother"),
        &*marie
    ));
    assert!(std::ptr::eq(
        irene.father().expect("Irene has father"),
        &*pierre
    ));
}

pub fn test_bool() {
    let marie = db::Persons::Marie;
    let pierre = db::Persons::Pierre;

    assert!(marie.woman());
    assert!(!pierre.woman());
}

pub fn test_float() {
    let marie = db::Persons::Marie;
    let pierre = db::Persons::Pierre;

    assert!(marie.score() == 1.0);
    assert!(pierre.score() == 2.1);
}
