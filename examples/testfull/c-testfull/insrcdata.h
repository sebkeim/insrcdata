// generated by insrcdata version 0.1.0

#ifndef INSRCDATA_INSRCDATA_H
#define INSRCDATA_INSRCDATA_H
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include "colobject.h"

typedef struct  {
    const char* name_;
    bool woman_;
    double score_;
    uint8_t spouse_;
    uint8_t father_;
    uint8_t mother_;
} person_t;
typedef struct { uint8_t* ptr; uint8_t* end; } person_iter_t;
extern const person_t* person_next(person_iter_t* idx);

typedef struct  {
    const char* text_;
} strencoding_t;
static unsigned const STRENCODING_TABLE_COUNT = 6;
extern const strencoding_t STRENCODING_TABLE[STRENCODING_TABLE_COUNT];
typedef struct { uint8_t* ptr; uint8_t* end; } strencoding_iter_t;
extern const strencoding_t* strencoding_next(strencoding_iter_t* idx);

typedef struct  {
    const char* name_;
    transformer_t* transformer_;
    const point_t* point_;
} lettercase_t;
typedef struct { uint8_t* ptr; uint8_t* end; } lettercase_iter_t;
extern const lettercase_t* lettercase_next(lettercase_iter_t* idx);

typedef struct  {
    uint32_t qid_;
    uint8_t object_;
} wikidata_t;
static unsigned const WIKIDATA_TABLE_COUNT = 3;
extern const wikidata_t WIKIDATA_TABLE[WIKIDATA_TABLE_COUNT];
typedef struct { uint8_t* ptr; uint8_t* end; } wikidata_iter_t;
extern const wikidata_t* wikidata_next(wikidata_iter_t* idx);

typedef enum {
     WIKIDATA_PERSON,
     WIKIDATA_LETTERCASE,
} wikidata_variant_t;
typedef struct {
    const wikidata_variant_t type;
    union {
     const person_t *person;
     const lettercase_t *lettercase;
    };
} wikidata_object_t;
typedef struct  {
    const char* lccn_;
    uint8_t object_;
} congress_t;
static unsigned const CONGRESS_TABLE_COUNT = 4;
extern const congress_t CONGRESS_TABLE[CONGRESS_TABLE_COUNT];
typedef struct { uint8_t* ptr; uint8_t* end; } congress_iter_t;
extern const congress_t* congress_next(congress_iter_t* idx);

typedef enum {
     CONGRESS_NONE,
     CONGRESS_PERSON,
     CONGRESS_LETTERCASE,
} congress_variant_t;
typedef struct {
    const congress_variant_t type;
    union {
     const person_t *person;
     const lettercase_t *lettercase;
    };
} congress_object_t;


// ------    
typedef enum {
    PERSONS_MARIE = 0,
    PERSONS_PIERRE = 1,
    PERSONS_IRENE = 2,
    PERSONS_FREDERIC = 3,
} persons_t;
const person_t* person_from_persons(persons_t label);
persons_t person_persons(const person_t *s);
            
static inline const char* person_name(const person_t* s) { return s->name_; }
static inline bool person_woman(const person_t* s) { return s->woman_; }
static inline double person_score(const person_t* s) { return s->score_; }
extern person_iter_t  person_score_range( double start, double stop);
extern const person_t* person_spouse(const person_t* s);
extern bool person_father(const person_t* s, const person_t** ptr);
extern bool person_mother(const person_t* s, const person_t** ptr);
extern wikidata_iter_t person_wdata(const person_t* s);
extern congress_iter_t person_congress(const person_t* s);


// ------    
static inline const char* strencoding_text(const strencoding_t* s) { return s->text_; }
extern strencoding_iter_t  strencoding_text_range( const char* start, const char* stop);


// ------    
typedef enum {
    LETTERCASES_CAPITAL = 0,
    LETTERCASES_UPPER = 1,
    LETTERCASES_LOWER = 2,
} lettercases_t;
const lettercase_t* lettercase_from_lettercases(lettercases_t label);
lettercases_t lettercase_lettercases(const lettercase_t *s);
            
static inline const char* lettercase_name(const lettercase_t* s) { return s->name_; }
static inline transformer_t* lettercase_transformer(const lettercase_t* s) { return s->transformer_; }
static inline const point_t* lettercase_point(const lettercase_t* s) { return s->point_; }
extern wikidata_iter_t lettercase_wdata2(const lettercase_t* s);
extern congress_iter_t lettercase_congress(const lettercase_t* s);


// ------    
static inline uint32_t wikidata_qid(const wikidata_t* s) { return s->qid_; }
extern wikidata_object_t wikidata_object(const wikidata_t* s);


// ------    
static inline const char* congress_lccn(const congress_t* s) { return s->lccn_; }
extern congress_object_t congress_object(const congress_t* s);

#endif //  INSRCDATA_H 
