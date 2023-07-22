use crate::insrcdata::Chapter;
use std::collections::HashSet;

// this sample show how to use insrcdata with a classification hierarchy

// order of row in the tables is important :
// we relly on a depth first table order for the `Chapter` table

mod insrcdata;

// print the fields contained by a hierarchy node and all it's children
fn print_chapter_content(code: &str) {
    // this algorithm could be optimized with public access to the `parent` and `chapter` fields
    // but is already quite fast for tables of average size
    // see C language sample for an alternative usable in larger tables

    // initialize he roots with searched code
    let roots = insrcdata::Chapter::code_range(code, code);
    let mut parents: HashSet<&Chapter> = HashSet::from_iter(roots);

    // collect all subchapters of roots
    for f in insrcdata::Chapter::array() {
        if parents.contains(&f.parent()) {
            parents.insert(f);
        }
    }

    // print oll the leaves corresponding to collected chapters
    for f in insrcdata::Leave::array() {
        if parents.contains(&f.chapter()) {
            println!("{}", f.title());
        }
    }
}

fn main() {
    // print all animals
    print_chapter_content("A");
}
