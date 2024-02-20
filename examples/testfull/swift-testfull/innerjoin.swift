//
//  fictolabel.swift
//  swift-testfull
//

import Foundation

// Inner join is when your join column reference a record in the same table
func test_innerjoin() {
  let marie = Person(ref: PERSONS_MARIE)
  let pierre = Person(ref: PERSONS_PIERRE)
  let irene = Person(ref: PERSONS_IRENE)
  let frederic = Person(ref: PERSONS_FREDERIC)

  assert(marie.name == "Marie Curie")
  assert(pierre.name == "Pierre Curie")
  assert(irene.name == "Irène Joliot-Curie")
  assert(frederic.name == "Frédéric Joliot-Curie")

  // inner join with 1-1 cardinality
  assert(pierre == marie.spouse)
  assert(marie == pierre.spouse)
  assert(frederic == irene.spouse)
  assert(irene == frederic.spouse)

  // inner join with 0-1 cardinality
  assert(marie.mother == nil)
  assert(marie.father == nil)
  assert(pierre.mother == nil)
  assert(pierre.father == nil)
  assert(frederic.mother == nil)
  assert(frederic.father == nil)
  assert(marie == irene.mother)
  assert(pierre == irene.father)
}

func test_bool() {
  let marie = Person(ref: PERSONS_MARIE)
  let pierre = Person(ref: PERSONS_PIERRE)

  assert(marie.woman)
  assert(!pierre.woman)
}

func test_float() {
  let marie = Person(ref: PERSONS_MARIE)
  let pierre = Person(ref: PERSONS_PIERRE)

  assert(marie.score == 1.0)
  assert(pierre.score == 2.1)

  // test closed range iterator
  // the iterator is stable : table order is preserved for equal values
  // we get all lower bound values
  let lower = Person.scoreRange(start: 2.1, stop: 3.2).map { $0.name }

  assert(
    lower == [
      "Pierre Curie",
      "Frédéric Joliot-Curie",
      "Irène Joliot-Curie",
    ])

  // we get all upper bound values
  let upper = Person.scoreRange(start: 1.0, stop: 2.1).map { $0.name }
  assert(upper == ["Marie Curie", "Pierre Curie", "Frédéric Joliot-Curie"])

  // test reversed
  assert(Array(Person.scoreRange(start: 2.0, stop: 1.9)).isEmpty)

  // under
  assert(Array(Person.scoreRange(start: 0.0, stop: 0.9)).isEmpty)
  assert(!Array(Person.scoreRange(start: 0.0, stop: 1.0)).isEmpty)

  // over
  assert(Array(Person.scoreRange(start: 10.0, stop: 9000.0)).isEmpty)
  assert(!Array(Person.scoreRange(start: 3.2, stop: 9000.0)).isEmpty)
}

// variant column
func test_variant_non_optional() {
  let q_marie = Wikidata(index: 0)
  let marie = Person(ref: PERSONS_MARIE)
  assert(
    WikidataObject.Person(marie) == q_marie.object
  )

  let q_lower = Wikidata(index: 1)
  let lower = Lettercase(ref: LETTERCASES_LOWER)
  assert(
    WikidataObject.Lettercase(lower) == q_lower.object
  )

  assert(Array(lower.wdata2).first == q_lower)

  let pierre = Person(ref: PERSONS_PIERRE)
  assert(Array(pierre.wdata).isEmpty)

}

func test_variant_optional() {
  let q_marie = Congress(index: 0)
  let marie = Person(ref: PERSONS_MARIE)
  assert(CongressObject.Person(marie) == q_marie.object)

  let q_lower = Congress(index: 1)
  let lower = Lettercase(ref: LETTERCASES_LOWER)
  assert(
    CongressObject.Lettercase(lower) == q_lower.object
  )

  assert(Array(lower.congress).first == q_lower)

  let pierre = Person(ref: PERSONS_PIERRE)
  assert(Array(pierre.congress).isEmpty)

  let q_france = Congress(index: 3)
  assert(CongressObject.None == q_france.object)
}

func innerjoin() {
  test_innerjoin()
  test_bool()
  test_float()
  test_variant_non_optional()
  test_variant_optional()
}
