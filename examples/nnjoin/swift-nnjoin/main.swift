//
//  main.swift
//  swift-nnjoin
//
// N-N join : many-to-many relationships
//

import Foundation

// insrcdata doesn't support natively many-to-many relationship between two tables.
// You can solve the problem with a third table which maintain two one-to-many relationships with the previous tables.
//
// This example maintain the many-to-many relationship between Client et Product with the Transaction table.

// All product for a client
func print_product_list(_ client: clients_t) {

  let info = Client(ref: client)
  print("*  \(info.name):")
  for transaction in info.transactions {
    print(" \(transaction.product.name)")
  }
  print()
}

// All clients for a product
func print_client_list(_ product: products_t) {
  let info = Product(ref: product)
  print("*  \(info.name):")
  for transaction in info.transactions {
    print(" \(transaction.client.name)")
  }
  print()
}

print("By clients")
print_product_list(CLIENTS_JOHN)
print_product_list(CLIENTS_ALIX)
print_product_list(CLIENTS_DAVID)

print("\nBy products")
print_client_list(PRODUCTS_APPLE)
print_client_list(PRODUCTS_BANANA)
print_client_list(PRODUCTS_PEACH)
print_client_list(PRODUCTS_CHERRY)
