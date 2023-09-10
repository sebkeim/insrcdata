// generated by insrcdata version 0.1.0

#ifndef INSRCDATA_INSRCDATA_H
#define INSRCDATA_INSRCDATA_H
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct  {
    const char* name_;
    uint8_t code_;
} region_t;
typedef struct { uint8_t* ptr; uint8_t* end; } region_iter_t;
extern const region_t* region_next(region_iter_t* idx);

typedef struct  {
    const char* name_;
    uint16_t code_;
    uint8_t region_;
} subregion_t;
typedef struct { uint8_t* ptr; uint8_t* end; } subregion_iter_t;
extern const subregion_t* subregion_next(subregion_iter_t* idx);

typedef struct  {
    const char* name_;
    const char* alpha2_;
    const char* alpha3_;
    uint16_t code_;
    uint8_t subregion_;
} country_t;
static unsigned const COUNTRY_TABLE_COUNT = 249;
extern const country_t COUNTRY_TABLE[COUNTRY_TABLE_COUNT];
typedef struct { uint8_t* ptr; uint8_t* end; } country_iter_t;
extern const country_t* country_next(country_iter_t* idx);



// ------    
static inline const char* region_name(const region_t* s) { return s->name_; }
static inline uint8_t region_code(const region_t* s) { return s->code_; }
extern subregion_iter_t region_subregions(const region_t* s);


// ------    
static inline const char* subregion_name(const subregion_t* s) { return s->name_; }
static inline uint16_t subregion_code(const subregion_t* s) { return s->code_; }
extern const region_t* subregion_region(const subregion_t* s);
extern country_iter_t subregion_countries(const subregion_t* s);


// ------    
typedef enum {
     COUNTRIES_ANTARCTICA = 8,
     COUNTRIES_BELGIUM = 21,
} countries_t;
const country_t* country_from_countries(countries_t label);
countries_t country_countries(const country_t *s);
            
static inline const char* country_name(const country_t* s) { return s->name_; }
static inline const char* country_alpha2(const country_t* s) { return s->alpha2_; }
static inline const char* country_alpha3(const country_t* s) { return s->alpha3_; }
extern country_iter_t  country_alpha3_range( const char* start, const char* stop);
static inline uint16_t country_code(const country_t* s) { return s->code_; }
extern country_iter_t  country_code_range( uint16_t start, uint16_t stop);
extern bool country_subregion(const country_t* s, const subregion_t** ptr);

#endif //  INSRCDATA_H 
