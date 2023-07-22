#include "insrcdata.h"
 
#include <assert.h>
#include <string.h>
#include <stdio.h>
  
// this sample show how to use insrcdata with a classification hierarchy

// order of row in the tables is important :
// we relly on a depth first table order for the `Chapter` table

 
// print the fields contained by a hierarchy node and all it's children
void print_chapter_content(const char* code)
{
      // for large tables, this algorith should perform better
      // than the implementation used by Rust sample
      
      // initialize the start with searched code
      chapter_iter_t  iter = chapter_code_range(code, code);
      const  chapter_t* start = chapter_next(&iter);
      if( start==NULL ){
            return;
      }
      
      // detect end of subtree
      const  chapter_t* end = start+1;
      for( ; end<CHAPTER_TABLE+CHAPTER_TABLE_COUNT; end++ ){
            if( chapter_parent(end)<start ){
                  break;
            }
      }
      
      // first chapter with content
      const  leave_t* leave = NULL;
      for( ;(leave==NULL) && (start<end); start++){
            leave_iter_t leaves = chapter_leaves(start);
            leave = leave_next(&leaves);
      }
      if( leave==NULL ){
            return;
      }
      
      // all content of hirarchy
      for( ; leave<LEAVE_TABLE+LEAVE_TABLE_COUNT; leave++ ){
            if( leave_chapter(leave)>=end ){
                  break;
            }
            printf("%s\n", leave_title(leave));
      }
}
 
 
int main()
{
      // print all animals
      print_chapter_content("A");
      
      return 0;
}
