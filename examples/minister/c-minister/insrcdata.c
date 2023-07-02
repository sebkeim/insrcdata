// generated by insrcdata version 0.1.0

#include "insrcdata.h"
#include <string.h>

static unsigned const MINISTER_TABLE_COUNT = 3;
static const minister_t MINISTER_TABLE[MINISTER_TABLE_COUNT] = {
   {"David Cameron", 1966, 0, },
   {"Gordon Brown", 1951, 0, },
   {"Romano Prodi", 1939, 1, },
};

const minister_t* minister_next(minister_iter_t* idx) { return idx->ptr<idx->end ? &MINISTER_TABLE[*idx->ptr++] : NULL; }
    

static unsigned const MINISTER_BIRTH_INDEX_COUNT  =  3;
static uint8_t MINISTER_BIRTH_INDEX   [MINISTER_BIRTH_INDEX_COUNT] = {
    2, 1, 0, 
};

static unsigned const MINISTER_COUNTRY_INDEX_COUNT  =  3;
static uint8_t MINISTER_COUNTRY_INDEX   [MINISTER_COUNTRY_INDEX_COUNT] = {
    0, 1, 2, 
};

const minister_t* minister_from_ministers(ministers_t label) {
    return &MINISTER_TABLE[label];
}
ministers_t minister_ministers(const minister_t *s) {
    return (ministers_t)(s-MINISTER_TABLE);
}

static unsigned const COUNTRY_TABLE_COUNT = 2;
static const country_t COUNTRY_TABLE[COUNTRY_TABLE_COUNT] = {
   {"GB", "United Kingdom", },
   {"IT", "Italy", },
};

const country_t* country_next(country_iter_t* idx) { return idx->ptr<idx->end ? &COUNTRY_TABLE[*idx->ptr++] : NULL; }
    

const country_t* country_from_countries(countries_t label) {
    return &COUNTRY_TABLE[label];
}
countries_t country_countries(const country_t *s) {
    return (countries_t)(s-COUNTRY_TABLE);
}

minister_iter_t  minister_birth_range( uint16_t start, uint16_t stop) {
    uint8_t* lo = MINISTER_BIRTH_INDEX;
    uint8_t*  hi = MINISTER_BIRTH_INDEX + MINISTER_BIRTH_INDEX_COUNT;
    while( lo < hi ){
        uint8_t*  mid = lo + ( hi-lo)/2;
        if( start>MINISTER_TABLE[*mid].birth_  ){
             lo = mid + 1;
        } else {
             hi = mid;
        }
    }

    uint8_t*  begin = lo;
    hi = MINISTER_BIRTH_INDEX + MINISTER_BIRTH_INDEX_COUNT;
    while( lo < hi ){
         uint8_t* mid = lo + ( hi-lo)/2;
        if( stop<MINISTER_TABLE[*mid].birth_  ){
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    minister_iter_t res = {  begin,  lo };
    return res;
}
const country_t* minister_country(const minister_t* s) { return &COUNTRY_TABLE[s->country_];}
minister_iter_t country_ministers(const country_t* s) {
    long cons = s - COUNTRY_TABLE;

    // bissect left
    uint8_t* lo = MINISTER_COUNTRY_INDEX;
    uint8_t* hi = MINISTER_COUNTRY_INDEX + MINISTER_COUNTRY_INDEX_COUNT;
   
    while( lo < hi ){
        uint8_t*  mid =  lo + ( hi-lo)/2;
        if ( cons > MINISTER_TABLE[*mid].country_ ) {
             lo = mid + 1;
        } else {
             hi = mid;
        }
    }
    uint8_t* begin = lo;

    // bissect-right
    hi = MINISTER_COUNTRY_INDEX +  MINISTER_COUNTRY_INDEX_COUNT;
    while( lo < hi ){
        uint8_t*  mid =  lo + ( hi-lo)/2;
        if( cons < MINISTER_TABLE[*mid].country_ )  {
            hi = mid;
        } else {
            lo = mid + 1;
        }
     }

    minister_iter_t res = {  begin,  lo };
    return res;
}

