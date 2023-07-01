// generated by insrcdata version 0.1.0

#ifndef INSRCDATA_INSRCDATA_H
#define INSRCDATA_INSRCDATA_H
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct  {
    const char* name_;
    uint16_t birth_;
    uint8_t country_;
} minister_t;
typedef struct { uint8_t* ptr; uint8_t* end; } minister_iter_t;
extern const minister_t* minister_next(minister_iter_t* idx);

typedef struct  {
    const char* code_;
    const char* name_;
} country_t;
typedef struct { uint8_t* ptr; uint8_t* end; } country_iter_t;
extern const country_t* country_next(country_iter_t* idx);



// ------    
typedef enum {
    MINISTERS_DAVID_CAMERON = 0,
    MINISTERS_GORDON_BROWN = 1,
    MINISTERS_ROMANO_PRODI = 2,
} ministers_t;
const minister_t* minister_from_ministers(ministers_t label);
ministers_t minister_ministers(const minister_t *s);
            
static inline const char* minister_name(const minister_t* s) { return s->name_; }
static inline uint16_t minister_birth(const minister_t* s) { return s->birth_; }
extern minister_iter_t  minister_birth_range( uint16_t start, uint16_t stop);
extern const country_t* minister_country(const minister_t* s);


// ------    
typedef enum {
    COUNTRIES_GB = 0,
    COUNTRIES_IT = 1,
} countries_t;
const country_t* country_from_countries(countries_t label);
countries_t country_countries(const country_t *s);
            
static inline const char* country_code(const country_t* s) { return s->code_; }
static inline const char* country_name(const country_t* s) { return s->name_; }
extern minister_iter_t country_ministers(const country_t* s);

#endif //  INSRCDATA_H 
