// generated by insrcdata version 0.3.0

#ifndef INSRCDATA_INSRCDATA_H
#define INSRCDATA_INSRCDATA_H
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct  {
    const char* title_;
} label_t;
static unsigned const LABEL_TABLE_COUNT = 10;
extern const label_t LABEL_TABLE[LABEL_TABLE_COUNT];



// ------    
typedef enum {
     LABELS_FOO = 0,
     LABELS_BAR = 1,
     LABELS_UPPER_CAMEL_CASE = 2,
     LABELS_LOWER_CAMEL_CASE = 3,
     LABELS_SNAKE_CASE = 4,
     LABELS_KEBAB_CASE = 5,
     LABELS_SHOUTY_SNAKE_CASE = 6,
     LABELS_TITLE_CASE = 7,
     LABELS_SHOUTY_KEBAB_CASE = 8,
     LABELS_TRAIN_CASE = 9,
} labels_t;
const label_t* label_from_labels(labels_t label);
labels_t label_labels(const label_t *s);
            
static inline const char* label_title(const label_t* s) { return s->title_; }

// swift bindings
static inline const label_t* LABEL_TABLE_PTR() { return LABEL_TABLE; }


#endif //  INSRCDATA_H 
