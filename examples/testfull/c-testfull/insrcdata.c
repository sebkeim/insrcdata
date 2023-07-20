// generated by insrcdata version 0.1.0

#include "insrcdata.h"
#include <string.h>

static unsigned const PERSON_TABLE_COUNT = 4;
static const person_t PERSON_TABLE[PERSON_TABLE_COUNT] = {
   {"Marie Curie", true, 1.0, 1, 0, 0, },
   {"Pierre Curie", false, 2.1, 0, 0, 0, },
   {"Irène Joliot-Curie", true, 3.2, 3, 2, 1, },
   {"Frédéric Joliot-Curie", false, 2.1, 2, 0, 0, },
};

const person_t* person_next(person_iter_t* idx) { return idx->ptr<idx->end ? &PERSON_TABLE[*idx->ptr++] : NULL; }
    

static unsigned const PERSON_SCORE_INDEX_COUNT  =  4;
static uint8_t PERSON_SCORE_INDEX   [PERSON_SCORE_INDEX_COUNT] = {
    0, 1, 3, 2, 
};

const person_t* person_from_persons(persons_t label) {
    return &PERSON_TABLE[label];
}
persons_t person_persons(const person_t *s) {
    return (persons_t)(s-PERSON_TABLE);
}

const strencoding_t STRENCODING_TABLE[STRENCODING_TABLE_COUNT] = {
   {"𝒾ň𝗌яčḓẚᵵᶏ : 𝔢ᶆḃ℮𝚍 ᶌ𝖔ừᵳ ⅆằƫⱥ", },
   {"hello", },
   {"κόσμε", },
   {"いろはにほへとちりぬるを", },
   {"éventuellement validé", },
   {"Да, но фальшивый экземпляр", },
};

const strencoding_t* strencoding_next(strencoding_iter_t* idx) { return idx->ptr<idx->end ? &STRENCODING_TABLE[*idx->ptr++] : NULL; }
    

static unsigned const STRENCODING_TEXT_INDEX_COUNT  =  6;
static uint8_t STRENCODING_TEXT_INDEX   [STRENCODING_TEXT_INDEX_COUNT] = {
    1, 4, 2, 5, 3, 0, 
};

static unsigned const LETTERCASE_TABLE_COUNT = 3;
static const lettercase_t LETTERCASE_TABLE[LETTERCASE_TABLE_COUNT] = {
   {"Capitalised case", make_capitalize, &POINT_ZERO, },
   {"Upper case", make_upper, &POINT_ONE, },
   {"Lower case", make_lower, &POINT_ONE, },
};

const lettercase_t* lettercase_from_lettercases(lettercases_t label) {
    return &LETTERCASE_TABLE[label];
}
lettercases_t lettercase_lettercases(const lettercase_t *s) {
    return (lettercases_t)(s-LETTERCASE_TABLE);
}

person_iter_t  person_score_range( double start, double stop) {
    uint8_t* lo = PERSON_SCORE_INDEX;
    uint8_t*  hi = PERSON_SCORE_INDEX + PERSON_SCORE_INDEX_COUNT;
    while( lo < hi ){
        uint8_t*  mid = lo + ( hi-lo)/2;
        if( start>PERSON_TABLE[*mid].score_  ){
             lo = mid + 1;
        } else {
             hi = mid;
        }
    }

    uint8_t*  begin = lo;
    hi = PERSON_SCORE_INDEX + PERSON_SCORE_INDEX_COUNT;
    while( lo < hi ){
         uint8_t* mid = lo + ( hi-lo)/2;
        if( stop<PERSON_TABLE[*mid].score_  ){
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    person_iter_t res = {  begin,  lo };
    return res;
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
strencoding_iter_t  strencoding_text_range( const char* start, const char* stop) {
    uint8_t* lo = STRENCODING_TEXT_INDEX;
    uint8_t*  hi = STRENCODING_TEXT_INDEX + STRENCODING_TEXT_INDEX_COUNT;
    while( lo < hi ){
        uint8_t*  mid = lo + ( hi-lo)/2;
        if( strcmp(start,STRENCODING_TABLE[*mid].text_ )>0 ){
             lo = mid + 1;
        } else {
             hi = mid;
        }
    }

    uint8_t*  begin = lo;
    hi = STRENCODING_TEXT_INDEX + STRENCODING_TEXT_INDEX_COUNT;
    while( lo < hi ){
         uint8_t* mid = lo + ( hi-lo)/2;
        if( strcmp(stop,STRENCODING_TABLE[*mid].text_ )<0 ){
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    strencoding_iter_t res = {  begin,  lo };
    return res;
}
