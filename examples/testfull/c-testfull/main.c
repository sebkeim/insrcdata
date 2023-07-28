#include "insrcdata.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>
#include <ctype.h>

// Inner join is when your join column reference a record in the same table
void  test_innerjoin(void) {
      const person_t* marie    = person_from_persons(PERSONS_MARIE   );
      const person_t* pierre   = person_from_persons(PERSONS_PIERRE  );
      const person_t* irene    = person_from_persons(PERSONS_IRENE   );
      const person_t* frederic = person_from_persons(PERSONS_FREDERIC);
      
      assert(!strcmp(person_name(marie)    , "Marie Curie"));
      assert(!strcmp(person_name(pierre)   , "Pierre Curie"));
      assert(!strcmp(person_name(irene)    , "Irène Joliot-Curie"));
      assert(!strcmp(person_name(frederic) , "Frédéric Joliot-Curie"));
      
      // inner join with 1-1 cardinality
      assert(person_spouse(marie   ) == pierre);
      assert(person_spouse(pierre  ) == marie);
      assert(person_spouse(irene   ) == frederic);
      assert(person_spouse(frederic) == irene);
      
      // inner join with 0-1 cardinality
      const person_t* parent = NULL;
      assert(!person_mother(marie, &parent));
      assert(!person_father(marie, &parent));
      assert(!person_mother(pierre, &parent));
      assert(!person_father(pierre, &parent));
      assert(!person_mother(frederic, &parent));
      assert(!person_father(frederic, &parent));
      
      assert(person_mother(irene, &parent) && parent==marie);
      assert(person_father(irene, &parent) && parent==pierre);
}

void test_bool(void) {
      const person_t* marie    = person_from_persons(PERSONS_MARIE);
      const person_t* pierre   = person_from_persons(PERSONS_PIERRE);
   
      assert(person_woman(marie));
      assert(!person_woman(pierre));
    
      /*
       clippy linter is triggered by the Rust code generated for this function
    
      // test iterator
      person_iter_t iter = person_woman_range(true, true);
      assert( person_next(&iter)==person_from_persons(PERSONS_MARIE) );
      assert( person_next(&iter)==person_from_persons(PERSONS_IRENE) );
      assert( person_next(&iter)==NULL );
       */
}

void test_float(void) {
      const person_t* marie    = person_from_persons(PERSONS_MARIE);
      const person_t* pierre   = person_from_persons(PERSONS_PIERRE);
   
      assert(person_score(marie)==1.0);
      assert(person_score(pierre)==2.1);
      
      // test closedrange iterator
      person_iter_t iter = person_score_range(2.1, 3.2);
      assert( person_next(&iter)==person_from_persons(PERSONS_PIERRE) );
      assert( person_next(&iter)==person_from_persons(PERSONS_FREDERIC) );
      assert( person_next(&iter)==person_from_persons(PERSONS_IRENE) );
      assert( person_next(&iter)==NULL );
      
      iter = person_score_range(1.0, 2.1);
      assert( person_next(&iter)==person_from_persons(PERSONS_MARIE) );
      assert( person_next(&iter)==person_from_persons(PERSONS_PIERRE) );
      assert( person_next(&iter)==person_from_persons(PERSONS_FREDERIC) );
      assert( person_next(&iter)==NULL );
      
      // test reversed
      iter = person_score_range(2.0, 1.9);
      assert( person_next(&iter)==NULL );
    
      // under
      iter = person_score_range(0.0, 0.9);
      assert( person_next(&iter)==NULL );
      iter = person_score_range(0.0, 1.0);
      assert( person_next(&iter)==person_from_persons(PERSONS_MARIE)  );
      assert( person_next(&iter)==NULL );
      
      // over
      iter = person_score_range(10.0, 9000.0);
      assert( person_next(&iter)==NULL ); 
      iter = person_score_range(3.2, 9000.0);
      assert( person_next(&iter)==person_from_persons(PERSONS_IRENE) );
      assert( person_next(&iter)==NULL );
}

// Pattern matching : retrieve label from fic record reference
uint16_t nobel_year(const person_t* x)  {
      switch( person_persons(x) ){
            case PERSONS_MARIE    : return 1911;  // also in 1903
            case PERSONS_PIERRE   : return 1903;
            case PERSONS_IRENE    : return 1935;
            case PERSONS_FREDERIC : return 1935;
            default: return 0;
      }
}

void test_fictolabel(void) {
      const person_t* marie    = person_from_persons(PERSONS_MARIE);
      const person_t* irene    = person_from_persons(PERSONS_IRENE);
      
      assert(nobel_year(person_spouse(marie)) == 1903);
      assert(nobel_year(irene) == 1935);
}

// check string comparison for various encoded unicode strings
static const int REFSTRS_COUNT = 6;
static const char* REFSTRS[REFSTRS_COUNT] = {
      "𝒾ň𝗌яčḓẚᵵᶏ : 𝔢ᶆḃ℮𝚍 ᶌ𝖔ừᵳ ⅆằƫⱥ",
      "hello",
      "κόσμε",
      "いろはにほへとちりぬるを",
      "éventuellement validé",
      "Да, но фальшивый экземпляр",
};

void  test_strencoding(void) {
      for( const char** refstr=REFSTRS;  refstr<REFSTRS+REFSTRS_COUNT; refstr++ ){
            strencoding_iter_t iter = strencoding_text_range(*refstr, *refstr);
            const strencoding_t* row = strencoding_next(&iter);
            assert(row!=NULL);
            const char* text = strencoding_text(row);
            assert( strcmp(text, *refstr)==0 );
            assert(strencoding_next(&iter)==NULL);
    }
}


// object type column : reference to native objects
extern void make_capitalize(char* str) {
      if( *str ){
            *str = toupper(*str);
      }
}
extern void make_upper(char* str) {
      for( ;*str ; str++ ){
            *str = toupper(*str);
      }
}
extern void make_lower(char*str) {
      for( ;*str ; str++ ){
            *str = tolower(*str);
      }
}

const point_t POINT_ZERO = {.0, .0};
const point_t POINT_ONE = {1.0, 1.0};

void  test_colobject(void) {
      char  hello1[] = "hello";
      const lettercase_t* upper = lettercase_from_lettercases(LETTERCASES_UPPER);
      transformer_t* upper_transformer = lettercase_transformer(upper);
      upper_transformer(hello1);
      assert( strcmp(hello1, "HELLO")==0 );
      assert(  lettercase_point(upper)->x==1.0);
      
      char  hello2[] = "hello";
      const lettercase_t* capital = lettercase_from_lettercases(LETTERCASES_CAPITAL);
      transformer_t* capital_transformer = lettercase_transformer(capital);
      capital_transformer(hello2);
      assert( strcmp(hello2, "Hello")==0 );
      assert(  lettercase_point(capital)->x==0.0);
}

void test_variant_non_optional(void) {
      
      // matched
      const wikidata_t * q_marie = &WIKIDATA_TABLE[0];
      wikidata_object_t obj_marie = wikidata_object(q_marie);
      assert(obj_marie.type==WIKIDATA_PERSON);
      assert(obj_marie.person==person_from_persons(PERSONS_MARIE));

      const wikidata_t * q_lower = &WIKIDATA_TABLE[1];
      wikidata_object_t obj_lower = wikidata_object(q_lower);
      assert(obj_lower.type==WIKIDATA_LETTERCASE);
      assert(obj_lower.lettercase==lettercase_from_lettercases(LETTERCASES_LOWER));

      wikidata_iter_t lower_iter = lettercase_wdata2(lettercase_from_lettercases(LETTERCASES_LOWER));
      assert(wikidata_next(&lower_iter)==q_lower);
      assert(wikidata_next(&lower_iter)==NULL);
      
      wikidata_iter_t piere_iter = person_wdata(person_from_persons(PERSONS_PIERRE));
      assert(wikidata_next(&piere_iter)==NULL);
}

void test_variant_optional(void) {
      // matched
      const congress_t * q_marie = &CONGRESS_TABLE[0];
      congress_object_t obj_marie = congress_object(q_marie);
      assert(obj_marie.type==CONGRESS_PERSON);
      assert(obj_marie.person==person_from_persons(PERSONS_MARIE));

      const congress_t * q_lower = &CONGRESS_TABLE[1];
      congress_object_t obj_lower = congress_object(q_lower);
      assert(obj_lower.type==CONGRESS_LETTERCASE);
      assert(obj_lower.lettercase==lettercase_from_lettercases(LETTERCASES_LOWER));
       
      congress_iter_t lower_iter = lettercase_congress(lettercase_from_lettercases(LETTERCASES_LOWER));
      assert(congress_next(&lower_iter)==q_lower);
      assert(congress_next(&lower_iter)==NULL);
      
      congress_iter_t piere_iter = person_congress(person_from_persons(PERSONS_PIERRE));
      assert(congress_next(&piere_iter)==NULL);
      
      // not matched
      const congress_t * q_france = &CONGRESS_TABLE[3];
      congress_object_t obj_france = congress_object(q_france);
      assert(obj_france.type==CONGRESS_NONE);
}

int main(void) {
      // the join column reference a record in the same table
      test_innerjoin();
      
      // bool data type
      test_bool();
      
      // float data type
      test_float();
      
      // retrieve label from fic record reference
      test_fictolabel();
      
      // check string comparison for various encoded unicode strings
      test_strencoding();
      
      // object type column : reference to native objects
      test_colobject();
      
      // variant
      test_variant_non_optional();
      
      // variant with unmatched rows
      test_variant_optional();
      
      return 0;
}


// TODO : show how to use a second insrcdata database in the same project
