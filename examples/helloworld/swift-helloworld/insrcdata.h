// generated by insrcdata version 0.3.0

#ifndef INSRCDATA_INSRCDATA_H
#define INSRCDATA_INSRCDATA_H
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct  {
    const char* sentence_;
} hello_world_t;
static unsigned const HELLO_WORLD_TABLE_COUNT = 2;
extern const hello_world_t HELLO_WORLD_TABLE[HELLO_WORLD_TABLE_COUNT];



// ------    
static inline const char* hello_world_sentence(const hello_world_t* s) { return s->sentence_; }

// swift bindings
static inline const hello_world_t* HELLO_WORLD_TABLE_PTR() { return HELLO_WORLD_TABLE; }


#endif //  INSRCDATA_H 
