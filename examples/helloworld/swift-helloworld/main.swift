//
//  main.swift
//  swift-helloworld
//
// minimalistic sample that print the paradigmatic sentence in the console
// note the use of the indexed constructor to access all the row of the table
// this method is activated by the flag array in the config file insrcdata.toml

import Foundation

for i in 0..<HELLO_WORLD_TABLE_COUNT {
  let fic = HelloWorld(index: Int(i))
  print("\(fic.sentence)")
}
