#include "insrcdata.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>


// This sample show how to define a minimal project, that will only generate labels wich will
// be used to link to external data
// This model is also known as the mixed data model

// This table simulate external data
// In real life application this could be for sample user-edited values stored in a database

typedef struct  {
      uint16_t  key; // is a number corresponding to the value of insrcdata::Labels
      char*   title; // could be a user edited title
} Outdata;

static unsigned const OUT_TABLE_COUNT = 2;
static  Outdata OUT_TABLE[OUT_TABLE_COUNT] = {
      { 0,  "Foo is awesome",},
      { 1,  "Bar is quite cool",},
};


// access external data from label
Outdata* out_data(labels_t label) {
      
      // naive brute-force search for the first record with the corresponding key
      for( Outdata* item=OUT_TABLE; item<OUT_TABLE+OUT_TABLE_COUNT; item++ ){
            if( item->key == label ){
                  return item;
            }
      }
      
      // not found
      return NULL;
}

// Sample
static void demo(void) {
      // geting external data from label
      Outdata* extfoo = out_data(LABELS_FOO);
      if( extfoo ){
            printf("what I have to say about Foo is '%s' !\n", extfoo->title);
      } else {
            printf("sorry didn't found Foo\n");
      }
      
      Outdata* fic = &OUT_TABLE[1];
      
      // Checking for label in external data
      if( fic->key == LABELS_FOO ){
            printf("yes it's Foo !\n");
      } else {
            printf("sorry you missed Foo ...\n");
      }
      
      // Pattern matching; retrieve label from external data  key
      switch( fic->key )  {
            case LABELS_FOO :printf("this is Foo\n"); break;
            case LABELS_BAR : printf("this is Bar\n"); break;
            case LABELS_UPPER_CAMEL_CASE: printf("this is LABELS_UPPER_CAMEL_CASE\n");
            case LABELS_LOWER_CAMEL_CASE : printf("this is LABELS_LOWER_CAMEL_CASE\n");
            case LABELS_SNAKE_CASE : printf("this is LABELS_SNAKE_CASE\n");
            case LABELS_KEBAB_CASE : printf("this is LABELS_KEBAB_CASE\n");
            case LABELS_SHOUTY_SNAKE_CASE : printf("this is LABELS_SHOUTY_SNAKE_CASE\n");
            case LABELS_TITLE_CASE : printf("this is LABELS_TITLE_CASE\n");
            case LABELS_SHOUTY_KEBAB_CASE : printf("this is LABELS_SHOUTY_KEBAB_CASE\n");
            case LABELS_TRAIN_CASE: printf("this is LABELS_TRAIN_CASE\n");
      }
}

// start of non regression tests
// the code that follow this point is not intended to be used as sample
// and may be more difficult to read (but should still be correct)

static void test(void) {
      // external data from label
      Outdata* extfoo = out_data(LABELS_FOO);
      assert( extfoo!=NULL);
      assert(!strcmp(extfoo->title, "Foo is awesome"));
      
      // compile-time check for various case conversions in input file
      unsigned dumy =
      LABELS_UPPER_CAMEL_CASE +
      LABELS_LOWER_CAMEL_CASE  +
      LABELS_SNAKE_CASE  +
      LABELS_KEBAB_CASE  +
      LABELS_SHOUTY_SNAKE_CASE  +
      LABELS_TITLE_CASE  +
      LABELS_SHOUTY_KEBAB_CASE  +
      LABELS_TRAIN_CASE ;
      assert(dumy>0);
}

int main(void) {
      test();
      demo();
      return 0;
}
