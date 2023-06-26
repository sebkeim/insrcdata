// generated by insrcdata version 0.1.0

#ifndef INSRCDATA_INSRCDATA_H
#define INSRCDATA_INSRCDATA_H
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct  {
    char* name_;
    uint8_t code_;
} region_t;
static unsigned const REGION_TABLE_COUNT = 5;
extern const region_t REGION_TABLE[REGION_TABLE_COUNT];
typedef struct { uint8_t* ptr; uint8_t* end; } region_iter_t;
static inline const region_t* region_next(region_iter_t*  idx) { return idx->ptr<idx->end ? &REGION_TABLE[*idx->ptr++] : NULL; }

typedef struct  {
    char* name_;
    uint16_t code_;
    uint8_t region_;
} subregion_t;
static unsigned const SUBREGION_TABLE_COUNT = 17;
extern const subregion_t SUBREGION_TABLE[SUBREGION_TABLE_COUNT];
typedef struct { uint8_t* ptr; uint8_t* end; } subregion_iter_t;
static inline const subregion_t* subregion_next(subregion_iter_t*  idx) { return idx->ptr<idx->end ? &SUBREGION_TABLE[*idx->ptr++] : NULL; }

typedef struct  {
    char* name_;
    char* alpha2_;
    char* alpha3_;
    uint16_t code_;
    uint8_t subregion_;
} country_t;
static unsigned const COUNTRY_TABLE_COUNT = 249;
extern const country_t COUNTRY_TABLE[COUNTRY_TABLE_COUNT];
typedef struct { uint8_t* ptr; uint8_t* end; } country_iter_t;
static inline const country_t* country_next(country_iter_t*  idx) { return idx->ptr<idx->end ? &COUNTRY_TABLE[*idx->ptr++] : NULL; }



// ------    
static inline const char* region_name(const region_t* s) { return s->name_; }
static inline const uint8_t region_code(const region_t* s) { return s->code_; }
extern subregion_iter_t region_subregions(const region_t* s);

// ------    
static inline const char* subregion_name(const subregion_t* s) { return s->name_; }
static inline const uint16_t subregion_code(const subregion_t* s) { return s->code_; }
static inline const region_t* subregion_region(const subregion_t* s) { return &REGION_TABLE[s->region_];}
extern country_iter_t subregion_countries(const subregion_t* s);

// ------    
typedef enum {
    COUNTRIES_ANTARCTICA = 8,
    COUNTRIES_BELGIUM = 21,
} countries_t;
static inline const char* country_name(const country_t* s) { return s->name_; }
static inline const char* country_alpha2(const country_t* s) { return s->alpha2_; }
static inline const char* country_alpha3(const country_t* s) { return s->alpha3_; }
extern country_iter_t  country_alpha3_range( char* start, char* stop);
static inline const uint16_t country_code(const country_t* s) { return s->code_; }
extern country_iter_t  country_code_range( uint16_t start, uint16_t stop);
static inline bool country_subregion(const country_t* s, const subregion_t** ptr)
{ 
    if( s->subregion_) {
        *ptr = &SUBREGION_TABLE[s->subregion_-1];
        return true;
    }
    return false;
}
#endif //  INSRCDATA_H 
