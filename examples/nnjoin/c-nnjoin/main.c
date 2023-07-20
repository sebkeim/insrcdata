#include "insrcdata.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>


// N-N join : many-to-many relationships
//
// insrcdata doesn't support natively many-to-many relationship between two tables.
// You can solve the problem with a third table which maintain two one-to-many relationships with the previous tables.
//
// This example maintain the many-to-many relationship between Client et Product with the Transaction table.

// All product for a client
void print_product_list(clients_t label) {
      const client_t* client = client_from_clients(label);
      printf("*  %s:", client_name(client));
      
      transaction_iter_t iter = client_transactions(client);
      const transaction_t* transaction;
      while( (transaction=transaction_next(&iter)) ){
            printf(" %s", product_name(transaction_product(transaction)));
      }
      printf("\n");
}

// All clients for a product
void print_client_list(products_t label) {
      const product_t* product = product_from_products(label);
      printf("*  %s:", product_name(product));
      
      transaction_iter_t iter = product_transactions(product);
      const transaction_t* transaction;
      while( (transaction=transaction_next(&iter)) ){
            printf(" %s", client_name(transaction_client(transaction)));
      }
      printf("\n");
}

int main() {
      printf("By clients\n");
      print_product_list(CLIENTS_JOHN);
      print_product_list(CLIENTS_ALIX);
      print_product_list(CLIENTS_DAVID);
      
      printf("\nBy products\n");
      print_client_list(PRODUCTS_APPLE);
      print_client_list(PRODUCTS_BANANA);
      print_client_list(PRODUCTS_PEACH);
      print_client_list(PRODUCTS_CHERRY);
      
      return 0;
}
