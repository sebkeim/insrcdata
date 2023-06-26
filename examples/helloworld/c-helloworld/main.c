#include "insrcdata.h"
 
#include <assert.h>
#include <string.h>
#include <stdio.h>
 

// minimalistic sample that print the paradigmatic sentence in the console
// note the use of the method ::array() to access all the row of the table
// this method is activated by the corresponding flag in the config file insrcdata.toml
int main(){
      for( const hello_world_t* fic=HELLO_WORLD_TABLE; fic<HELLO_WORLD_TABLE+HELLO_WORLD_TABLE_COUNT; fic++ ) {
          printf("%s\n", hello_world_sentence(fic));
      }
      return 0;
}
