#include "insrcdata.h"
 
#include <assert.h>
#include <string.h>
#include <stdio.h>

// simulate rust trait by a function
static bool region_in_eurasia(const region_t* s) {
      int code = region_code(s);
      return code == 142 || code == 150 ;
}


// This function will print some information from a country
static void print_country(const country_t* country)
{
      //  countries may not have subregion(see ANTARTICA), so the field is optional
      subregion_t* subregion = NULL;
      if( country_subregion(country, &subregion) ){
            region_t* region = subregion_region(subregion);
            printf(
                   " %s ( ISO 3166‑1: %s ) in %s from %s\n",
                   country_name(country),
                   country_alpha3(country),
                   subregion_name(subregion),
                   region_name(region)
                   );
      } else {
            printf(
                   " %s ( ISO 3166‑1: %s ) has no subregion\n",
                   country_name(country),
                   country_alpha3(country)
                   );
            
      }
}


static void demo() {
      // row access by label
      country_t* belgium = &COUNTRY_TABLE[COUNTRIES_BELGIUM];
    
      printf("\n  infos for %s : \n", country_name(belgium));
      print_country(belgium);

      printf("\n  info for a country without subregion\n");
      print_country(&COUNTRY_TABLE[COUNTRIES_ANTARCTICA]);

      printf("\n  all countries with  alpha3 code starting by 'F'\n");
      country_t* country;
      country_iter_t iter = country_alpha3_range( "F", "G");
      while( (country = country_next(&iter)) ){
            print_country(country);
      }
   
      printf("\n  country with UNO code  136\n");
      iter = country_code_range(136, 136);
      while( (country = country_next(&iter)) ){
            print_country(country);
      }
     
      printf("\n  all countries with UNO code  in the range [100..112] \n");
      iter = country_code_range(100, 112);
      while( (country = country_next(&iter)) ){
            print_country(country);
      }
     
      printf("\n  all countries in Western Europe\n");
      subregion_t* west_europe = NULL;
      if( !country_subregion(belgium, &west_europe) ){
            assert(false); //"Belgium has a subregion"
      }
      iter = subregion_countries(west_europe);
      while( (country = country_next(&iter)) ){
            print_country(country);
      }
   
      printf("\n  all subregions in europe\n");
      region_t* europe = subregion_region(west_europe);
      subregion_iter_t sriter = region_subregions(europe);
      subregion_t* subregion;
      while( (subregion = subregion_next(&sriter)) ){
            printf("%s ( code : %d )\n", subregion_name(subregion), subregion_code(subregion));
      }
      
      printf("\n  the first 5 countries of the table\n");
      assert(COUNTRY_TABLE_COUNT>5);
      for( int i=0; i<5; i++) {
          print_country(&COUNTRY_TABLE[i]);
      }
}

// start of non regression tests
// the code that follow this point is not intended to be used as sample
// and may be more difficult to read (but should still be correct)


//  lookup by code
static bool alpha3_country(char* code, country_t** ptr) {
      country_iter_t iter = country_alpha3_range(code, code);
      country_t* country = country_next(&iter);
      if( country==NULL ){
            return false;
      }
      *ptr = country;
      return true;
}

static void test_sdn_sgp(char* start, char* stop){
      country_iter_t iter = country_alpha3_range( start, stop);
      country_t* country = country_next(&iter);
      assert( country!=NULL && strcmp(country_alpha3(country), "SDN")==0 ); // Sudan
      country = country_next(&iter);
      assert( country!=NULL && strcmp(country_alpha3(country), "SEN")==0 ); // Senegal
      country = country_next(&iter);
      assert( country!=NULL && strcmp(country_alpha3(country), "SGP")==0 ); // Singapore
      country = country_next(&iter);
      assert( country==NULL  );

}

static void test(void) {
      
      country_t* belgium = &COUNTRY_TABLE[COUNTRIES_BELGIUM];
      assert( strcmp(country_name(belgium), "Belgium")==0 );
      assert( strcmp(country_alpha3(belgium), "BEL")==0 );
      assert( strcmp(country_alpha2(belgium), "BE")==0 );
      assert( country_code(belgium) == 56 );
      
      subregion_t* west_europe = NULL;
      if( !country_subregion(belgium, &west_europe) ){
            assert(false); //"Belgium has a subregion"
      }
      region_t* europe = subregion_region(west_europe);
      assert( region_in_eurasia(europe) );

      country_iter_t iter = country_alpha3_range( "BEN", "ZZZ");
      country_t* benin = country_next(&iter);
      assert( benin!=NULL );
      assert( strcmp(country_name(benin), "Benin")==0 );
      
      subregion_t* subsahara = NULL;
      if( !country_subregion(benin, &subsahara) ){
            assert(false); //"Benin has a subregion"
      }
      assert( subregion_code(subsahara) == 202 );
      
      
      region_t*  africa = subregion_region(subsahara);
      assert( region_code(africa) == 2 );
      assert(!region_in_eurasia(africa));
      
      // check reverse join 0..1
      bool benin_in_subsahara = false;
      iter = subregion_countries(subsahara);
      country_t* country;
      while( (country = country_next(&iter)) ){
            if( country==benin ){
                  benin_in_subsahara = true;
            }
      }
      assert( benin_in_subsahara );
      
      // check reverse join 1..1
      bool subsahara_in_africa = false;
      subregion_iter_t  sriter = region_subregions(africa);
      subregion_t* subregion;
      while( (subregion = subregion_next(&sriter)) ){
            if( subregion==subsahara ){
                  subsahara_in_africa = true;
            }
      }
      assert( subsahara_in_africa );
      
      //  lookup by code
      country_t* france;
      if( !alpha3_country("FRA", &france) ){
            assert(false); //FRA is france iso code
      }
      assert( strcmp(country_name(france), "France")==0 );
      
      country_t* unknown;
      assert( !alpha3_country("XYZ", &unknown));
      
      // test open and closed iter range
      test_sdn_sgp("SDN", "SGP");
      test_sdn_sgp("SDM", "SGP");
      test_sdn_sgp("SDN", "SGQ");
      test_sdn_sgp("SDM", "SGQ");
}

int main(){
      test();
      demo();
      return 0;
}
