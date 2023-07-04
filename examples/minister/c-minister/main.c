#include "insrcdata.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>

// overview of insrcdata

int main(void)
{
      // get individual elements
      const minister_t* g_brown = minister_from_ministers(MINISTERS_GORDON_BROWN);
      
      // access it's attributes
      printf("%s was born in %d.\n", minister_name(g_brown), minister_birth(g_brown));
      
      // navigate between linked table
      const country_t* country = minister_country(g_brown);
      printf("He was prime minister of %s\n", country_name(country));
      
      // perform indexed searches
      minister_iter_t iter = minister_birth_range(1900, 1960);
      const minister_t* minister;
      while( (minister = minister_next(&iter)) ){
            printf("%s\n", minister_name(minister));
      }
      
      // perform reverse lookup between tables
      const country_t* gb = country_from_countries(COUNTRIES_GB);
      iter = country_ministers(gb);
      while( (minister = minister_next(&iter)) ){
            printf("%s\n", minister_name(minister));
      }
      
}
