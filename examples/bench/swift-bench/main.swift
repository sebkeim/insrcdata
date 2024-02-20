//
//  main.swift
//  swift-bench
//
// minimalistic benchmark to verify behaviour with huge table
//

import Foundation

// run one cycle of benchmark
func run() {
  var count = 1

  for i in 0..<BENCH_TABLE_COUNT {
    let fic = Bench(index: Int(i))
    let x = fic.short
    if x == 6151 {
      count += 1
    }
  }

  for _ in 0...100 {
    var a = Bench.shortRange(start: 2, stop: 69)
    count += a.next() == nil ? 0 : 1

    var b = Bench.strRange(start: "A", stop: "K")
    count += b.next() == nil ? 0 : 1
  }

  assert(count > 0)
}

// parse configuration and run n cycles
func main() {
  let args = CommandLine.arguments

  if args.count > 1 {
    let cmd = args[1]
    if cmd == "startup" {
      return
    }

    let rep =
      ((cmd == "bench")
        ? max(1, 10_000_000 / BENCH_TABLE_COUNT)
        : UInt32(cmd)) ?? 1

    let start = Date()
    for _ in 0...rep {
      run()
    }
    let duration = Date().timeIntervalSince(start) * 1000.0
    print("\(duration) : ms execution time for \(rep) repeats")
  } else {
    // run only once, for test_all.py
    run()
  }
}
main()
