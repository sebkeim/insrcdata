use std::collections::HashSet;

// this sample show how to use insrcdata with a classification hierarchy

// order of row in the tables is important :
// we relly on a depth first table order for the `Chapter` table

#[allow(dead_code)]
#[allow(unused_variables)]
mod insrcdata;

// print the fields contained by a hierarchy node and all it's children
fn print_chapter_content(code: &str) {
    // this algorithm could be optimized with public access to the `parent` and `chapter` fields
    // but is already quite fast for tables of average size
    // see C language sample for an alternative usable in larger tables

    // initialize the roots with searched code
    let mut parents: HashSet<&str> = HashSet::new();
    parents.insert(code);

    // collect all subchapters of roots
    for f in insrcdata::Chapter::array() {
        if parents.contains(&f.parent().code()) {
            parents.insert(f.code());
        }
    }

    // print all the leaves corresponding to collected chapters
    for f in insrcdata::Leave::array() {
        if parents.contains(&f.chapter().code()) {
            println!("{}", f.title());
        }
    }
}

fn main() {
    // print all animals
    print_chapter_content("A");
}
