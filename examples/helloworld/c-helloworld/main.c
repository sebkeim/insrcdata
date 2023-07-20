#include "insrcdata.h"
 
#include <assert.h>
#include <string.h>
#include <stdio.h>
 
 
int main(){
      for( const hello_world_t* fic=HELLO_WORLD_TABLE; fic<HELLO_WORLD_TABLE+HELLO_WORLD_TABLE_COUNT; fic++ ) {
          printf("%s\n", hello_world_sentence(fic));
      }
      return 0;
}
