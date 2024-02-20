//
//  strencoding.swift
//  swift-testfull
//
//

import Foundation

let REFSTRS = [
  "𝒾ň𝗌яčḓẚᵵᶏ : 𝔢ᶆḃ℮𝚍 ᶌ𝖔ừᵳ ⅆằƫⱥ",
  "hello",
  "κόσμε",
  "いろはにほへとちりぬるを",
  "éventuellement validé",
  "Да, но фальшивый экземпляр",
]

// check string comparison for various encoded unicode strings
func test_strencoding() {
  for refstr in REFSTRS {
    let iter = Strencoding.textRange(start: refstr, stop: refstr)
    assert(iter.map { $0.text } == [refstr])

  }
}
func strencoding() {
  test_strencoding()
}
