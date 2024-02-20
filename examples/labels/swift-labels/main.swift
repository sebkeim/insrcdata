//
//  main.swift
//  swift-labels
//

/// This sample show how to define a minimal project, that will only generate labels wich will
/// be used to link to external data
/// This model is also known as the mixed data model

import Foundation

/// This table simulate external data
/// In real life application this could be for sample user-edited values stored in a database
struct Outdata {
  var key: Int
  var title: String
}

let OUT_TABLE = [
  Outdata(
    key: 0,
    title: "Foo is awesome"
  ),
  Outdata(
    key: 1,
    title: "Bar is quite cool"
  ),
]

// Access external data from label
func out_data(_ label: labels_t) -> Outdata? {
  // convert searched label in the format of the key uded by outdata
  let key = label.rawValue

  // naive brute-force search for the first record with the corresponding key
  for item in OUT_TABLE {
    if item.key == key {
      return item
    }
  }

  // not found
  return nil
}

// Sample
func demo() {
  // geting external data from label
  if let foo = out_data(LABELS_FOO) {
    print("what I have to say about Foo is '\(foo.title)' !")
  } else {
    print("sorry didn't found Foo")
  }

  let fic = OUT_TABLE[1]

  // Checking for label in external data
  if fic.key == LABELS_FOO.rawValue {
    print("yes it's Foo !")
  } else {
    print("sorry you missed Foo ...")
  }

  //  in this sample external data  key is row index
  let res = Label(index: fic.key)

  // match label from table row field
  // make also a compile-time check for various case conversions in input file
  switch res.labels {
  case LABELS_FOO: print("this is Foo")
  case LABELS_BAR: print("this is Bar")
  case LABELS_UPPER_CAMEL_CASE: print("This is UpperCamelCase")
  case LABELS_LOWER_CAMEL_CASE: print("This is LowerCamelCase")
  case LABELS_SNAKE_CASE: print("This is SnakeCase")
  case LABELS_KEBAB_CASE: print("This is KebabCase")
  case LABELS_SHOUTY_SNAKE_CASE: print("This is ShoutySnakeCas")
  case LABELS_TITLE_CASE: print("This is TitleCase")
  case LABELS_SHOUTY_KEBAB_CASE: print("This is ShoutyKebabCas")
  case LABELS_TRAIN_CASE: print("This is TrainCase")
  default: print("not found....")
  }
}

// start of non regression tests
// the code that follow this point is not intended to be used as sample
// and may be more difficult to read (but should still be correct)

func test() {
  // external data from label
  let extfoo = OUT_TABLE[Int(LABELS_FOO.rawValue)]
  assert(extfoo.title == "Foo is awesome")
}

test()
demo()
