#include "insrcdata.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>

// Inner join is when your join column reference a record in the same table
void  test_innerjoin(void) {
      const person_t* marie    = persons_person(PERSONS_MARIE   );
      const person_t* pierre   = persons_person(PERSONS_PIERRE  );
      const person_t* irene    = persons_person(PERSONS_IRENE   );
      const person_t* frederic = persons_person(PERSONS_FREDERIC);
      
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
      const person_t* marie    = persons_person(PERSONS_MARIE);
      const person_t* irene    = persons_person(PERSONS_IRENE);
      
      assert(nobel_year(person_spouse(marie)) == 1903);
      assert(nobel_year(irene) == 1935);
}


int main(void) {
      // the join column reference a record in the same table
      test_innerjoin();
      
      //  retrieve label from fic record reference
      test_fictolabel();
      return 0;
}


// TODO : show how to use a second insrcdata database in the same project
