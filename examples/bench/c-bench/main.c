#include "insrcdata.h"

// minimalistic benchmark to verify behaviour with huge table

#include <assert.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

// run one cycle of benchmark
void run(void)
{
      int count = 1;
      
      for(int i=0; i<BENCH_TABLE_COUNT; i++){
            uint32_t x = bench_short(BENCH_TABLE+i);
            if( x == 6151 ){
                  count += 1;
            }
      }
      
      for(int i=0; i<100; i++){
            bench_iter_t it =  bench_short_range( 2, 69);
            count += bench_next(&it)!=NULL;
            
            it =  bench_str_range("A", "K");
            count += bench_next(&it)!=NULL;
      }
      
      assert(count > 0);
}

// parse configuration and run n cycles
int main(int argc, char* argv[])
{
      if( argc > 1 ){
            char* cmd = argv[1];
            if( strcmp(cmd,"startup")==0  ){
                  return 0;
            }
            
            int repeat = 10000000/BENCH_TABLE_COUNT;
            if( strcmp(cmd,"bench")!=0  ){
                  repeat = atoi(cmd);
            }
            
            struct timeval stop, start;
            gettimeofday(&start, NULL);
            for(int i=0; i<repeat; i++){
                  run();
            }
            
            gettimeofday(&stop, NULL);
            long elapsed = ((stop.tv_sec - start.tv_sec) * 1000000 + stop.tv_usec - start.tv_usec)/1000;
            printf("%ld : ms execution time\n", elapsed);
      } else {
            // run only once, for test_all.py
            run();
      }
      return 0;
}
