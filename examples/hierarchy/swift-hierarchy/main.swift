//
//  main.swift
//  swift-hierarchy
//
// this sample show how to use insrcdata with a classification hierarchy

// order of row in the tables is important :
// we relly on a depth first table order for the `Chapter` table

// print the fields contained by a hierarchy node and all it's children
func print_chapter_content(code: String) {
  // this algorithm could be optimized with public access to the `parent` and `chapter` fields
  // but is already quite fast for tables of average size
  // see C language sample for an alternative usable in larger tables

  // initialize he roots with searched code
  var parents = Set([code])

  // collect all subchapters of roots
  for i in 0..<CHAPTER_TABLE_COUNT {
    let f = Chapter(index: Int(i))
    if parents.contains(f.parent.code) {
      parents.insert(f.code)
    }
  }

  // print oll the leaves corresponding to collected chapters
  for j in 0..<LEAVE_TABLE_COUNT {
    let f = Leave(index: Int(j))
    if parents.contains(f.chapter.code) {
      print(f.title)
    }
  }
}

// print all animals
print_chapter_content(code: "A")
