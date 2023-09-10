// generated by insrcdata version 0.1.0

#ifndef INSRCDATA_INSRCDATA_H
#define INSRCDATA_INSRCDATA_H
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct  {
    const char* name_;
} client_t;
typedef struct { uint8_t* ptr; uint8_t* end; } client_iter_t;
extern const client_t* client_next(client_iter_t* idx);

typedef struct  {
    const char* name_;
} product_t;
typedef struct { uint8_t* ptr; uint8_t* end; } product_iter_t;
extern const product_t* product_next(product_iter_t* idx);

typedef struct  {
    uint8_t client_;
    uint8_t product_;
} transaction_t;
typedef struct { uint8_t* ptr; uint8_t* end; } transaction_iter_t;
extern const transaction_t* transaction_next(transaction_iter_t* idx);



// ------    
typedef enum {
     CLIENTS_JOHN = 0,
     CLIENTS_ALIX = 1,
     CLIENTS_DAVID = 2,
} clients_t;
const client_t* client_from_clients(clients_t label);
clients_t client_clients(const client_t *s);
            
static inline const char* client_name(const client_t* s) { return s->name_; }
extern transaction_iter_t client_transactions(const client_t* s);


// ------    
typedef enum {
     PRODUCTS_APPLE = 0,
     PRODUCTS_BANANA = 1,
     PRODUCTS_PEACH = 2,
     PRODUCTS_CHERRY = 3,
} products_t;
const product_t* product_from_products(products_t label);
products_t product_products(const product_t *s);
            
static inline const char* product_name(const product_t* s) { return s->name_; }
extern transaction_iter_t product_transactions(const product_t* s);


// ------    
extern const client_t* transaction_client(const transaction_t* s);
extern const product_t* transaction_product(const transaction_t* s);

#endif //  INSRCDATA_H 
