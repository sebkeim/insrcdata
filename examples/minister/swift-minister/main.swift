//
//  main.swift
//  swift-minister
// overview of insrcdata
//

import Foundation

let g_brown = Minister(ref: MINISTERS_GORDON_BROWN)

// access it's attributes
print("\(g_brown.name) was born in \(g_brown.birth).")

// navigate between linked table
print("He was prime minister of \(g_brown.country.name).")

// perform indexed searches
for minister in Minister.birthRange(start: 1900, stop: 1960) {
  print(minister.name)
}

// perform reverse lookup between tables
let uk = Country(ref: COUNTRIES_UK)
for minister in uk.ministers {
  print(minister.name)
}
