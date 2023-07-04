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
      const person_t* marie    = person_from_persons(PERSONS_MARIE   );
      const person_t* pierre   = person_from_persons(PERSONS_PIERRE  );
   
      assert(person_woman(marie));
      assert(!person_woman(pierre));
}

void test_float(void) {
      const person_t* marie    = person_from_persons(PERSONS_MARIE   );
      const person_t* pierre   = person_from_persons(PERSONS_PIERRE  );
   
      assert(person_score(marie)==1.0);
      assert(person_score(pierre)==2.1);
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
      return 0;
}


// TODO : show how to use a second insrcdata database in the same project
