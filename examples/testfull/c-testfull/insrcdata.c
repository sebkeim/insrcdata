// generated by insrcdata version 0.1.0

#include "insrcdata.h"
#include <string.h> 

static unsigned const PERSON_TABLE_COUNT = 4;
static const person_t PERSON_TABLE[PERSON_TABLE_COUNT] = {
   {"Marie Curie", 1, 0, 0, },
   {"Pierre Curie", 0, 0, 0, },
   {"Irène Joliot-Curie", 3, 2, 1, },
   {"Frédéric Joliot-Curie", 2, 0, 0, },
};

const person_t* person_from_persons(persons_t label) {
    return &PERSON_TABLE[label];
}
persons_t person_persons(const person_t *s) {
    return s-PERSON_TABLE;
}

const person_t* person_spouse(const person_t* s) { return &PERSON_TABLE[s->spouse_];}
bool person_father(const person_t* s, const person_t** ptr) {
    if( s->father_) {
        *ptr = &PERSON_TABLE[s->father_-1];
        return true;
    }
    return false;
}
bool person_mother(const person_t* s, const person_t** ptr) {
    if( s->mother_) {
        *ptr = &PERSON_TABLE[s->mother_-1];
        return true;
    }
    return false;
}
