// generated by insrcdata version 0.1.0

#ifndef INSRCDATA_INSRCDATA_H
#define INSRCDATA_INSRCDATA_H
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct  {
    char* sentence_;
} hello_world_t;
static unsigned const HELLO_WORLD_TABLE_COUNT = 2;
extern hello_world_t HELLO_WORLD_TABLE[HELLO_WORLD_TABLE_COUNT];



// ------    
static inline char* hello_world_sentence(const hello_world_t* s) { return s->sentence_; }
#endif //  INSRCDATA_H
