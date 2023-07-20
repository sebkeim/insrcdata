#include "insrcdata.h"
 
#include <assert.h>
#include <string.h>
#include <stdio.h>
 
// ================================================================================================
// optional join strategy use optional join tables to store the values
// this the most robust approach but has some drawbacks :
//  - data size may become larger, in the absence of duplicate values
//  - range indexing become unreasonably complicated
// ================================================================================================

// getter for optional float value
static bool optjoin_score(const optjoin_t* s, float* value){
      const score_t* ptr;
      if( optjoin_score_join(s, &ptr) ){
            *value = score_value(ptr);
            return true;
      }
      return false;
}

// getter for optional uint16_t value
static bool optjoin_count(const optjoin_t* s, uint16_t* value){
      const count_t* ptr;
      if( optjoin_count_join(s, &ptr) ){
            *value = count_value(ptr);
            return true;
      }
      return false;
}
 
// print the content of a row
static void optjoin_print(optjoins_t label, char* name){
      const optjoin_t* s = optjoin_from_optjoins(label);
      printf("%s", name);
      float score;
      if( optjoin_score(s, &score) ){
            printf(" score=%f", score);
      }
      uint16_t count;
      if( optjoin_count(s, &count) ){
            printf(" count=%d", count);
      }
      printf("\n");
}

//  optjoin strategy usage sample
static void optjoin_sample() {
      printf("    Optional join strategy\n");
      optjoin_print(OPTJOINS_FILLED, "Filled");
      optjoin_print(OPTJOINS_EMPTY, "Empty");
      printf("\n");
}


// ================================================================================================
// ad-hoc strategy use placeholder to represent 'no value'
// ================================================================================================
 
// placeholder values that represent 'no value'
// we must be certain that theses values will never be needed for real data
static const float SCORE_EMPTY = -1.0;
static const uint16_t COUNT_EMPTY = 0;

// getter for optional float value
static bool adhoc_score(const adhoc_t* s, float* value){
      float data = adhoc_score_data(s);
      if( data==SCORE_EMPTY ){
            return false;
      }
      *value = data;
      return true;
}

// getter for optional uint16_t value
static bool adhoc_count(const adhoc_t* s, uint16_t* value){
      uint16_t data = adhoc_count_data(s);
      if( data==COUNT_EMPTY ){
            return false;
      }
      *value = data;
      return true;
}


// print the content of a row
static void adhoc_print(adhocs_t label, char* name){
      const adhoc_t* s = adhoc_from_adhocs(label);
      printf("%s", name);
      float score;
      if( adhoc_score(s, &score) ){
            printf(" score=%f", score);
      }
      uint16_t count;
      if( adhoc_count(s, &count) ){
            printf(" count=%d", count);
      }
      printf("\n");
}

//  ad-hoc strategy usage sample
static void adhoc_sample() {
      printf("    Optional join strategy\n");
      adhoc_print(ADHOCS_FILLED, "Filled");
      adhoc_print(ADHOCS_EMPTY, "Empty");
      printf("\n");
}

// ================================================================================================
//
// ================================================================================================
int main(){
      optjoin_sample();
      adhoc_sample();
      return 0;
}
 
