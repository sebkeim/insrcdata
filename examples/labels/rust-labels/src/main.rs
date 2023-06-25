use crate::insrcdata::Labels;

mod insrcdata;

/// This sample show how to define a minimal project, that will only generate labels wich will
/// be used to link to external data
/// This model is also known as the mixed data model
/// TODO: struct Inexata and TABLE are useless and must not be generated

/// This table simulate external data
/// In real life application this could be for sample user-edited values stored in a database

#[derive(Debug, Copy, Clone)]
struct Outdata {
    key: u16,            // is a number corresponding to the value of insrcdata::Labels
    title: &'static str, // could be a user edited title
}

static OUT_TABLE: [Outdata; 2] = [
    Outdata {
        key: 0,
        title: "Foo is awesome",
    },
    Outdata {
        key: 1,
        title: "Bar is quite cool",
    },
];

// access external data from label
fn out_data(label: insrcdata::Labels) -> Option<Outdata> {
    // convert searched label in the format of the key uded by outdata
    let key = label as u16;

    // naive brute-force search for the first record with the corresponding key
    for item in OUT_TABLE.iter() {
        if item.key == key {
            return Option::Some(*item);
        }
    }

    // not found
    None
}

// Sample
fn demo() {
    // geting external data from label
    let extfoo = out_data(insrcdata::Labels::Foo);

    match extfoo {
        Some(fic) => println!("what I have to say about Foo is '{}' !", fic.title),
        None => println!("sorry didn't found Foo"),
    }

    let fic = OUT_TABLE[1];

    // Checking for label in external data
    if fic.key == Labels::Foo as u16 {
        println!("yes it's Foo !");
    } else {
        println!("sorry you missed Foo ...");
    }

    // Pattern matching ; retrieve label from external data  key
    // In rust there is [no builtin way to retrieve enum from it's numeric value](https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html)
    //  you have to use  [double match](https://stackoverflow.com/questions/68677383/match-integers-against-enum-cases-without-a-double-match-statement-in-rust)
    //  making pattern matching will be quite inefficient if there is many values to check
    match fic.key {
        x if x == insrcdata::Labels::Foo as u16 => println!("this is Foo"),
        x if x == insrcdata::Labels::Bar as u16 => println!("this is Bar"),
        _ => println!("!unknown!"),
    }
    // TODO: implement [num_enum](https://crates.io/crates/num_enum) for efficient conversion from external key to Labels
}

// start of non regression tests
// the code that follow this point is not intended to be used as sample
// and may be more difficult to read (but should still be correct)

fn test() {
    // external data from label
    let extfoo = out_data(insrcdata::Labels::Foo).expect("overflow");
    assert!(extfoo.title == "Foo is awesome");

    // compile-time check for various case conversions in input file
    let converted_cases = vec![
        insrcdata::Labels::UpperCamelCase,
        insrcdata::Labels::LowerCamelCase,
        insrcdata::Labels::SnakeCase,
        insrcdata::Labels::KebabCase,
        insrcdata::Labels::ShoutySnakeCase,
        insrcdata::Labels::TitleCase,
        insrcdata::Labels::ShoutyKebabCase,
        insrcdata::Labels::TrainCase,
    ];
    assert!(!converted_cases.is_empty()); // silent warning unused
}

fn main() {
    test();
    demo();
}
