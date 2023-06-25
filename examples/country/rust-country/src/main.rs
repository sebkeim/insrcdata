//
//   data come from https://raw.githubusercontent.com/lukes/ISO-3166-Countries-with-Regional-Codes/master/all/all.csv
//        added label colun

mod insrcdata;

mod country {

    // we put our code in a module : this is needed to implement the trait Region in insrcdata
    // generated code

    use crate::insrcdata as db;

    // Use trait to add custom methods to generated structures
    // tip : insrcdata -trait <yourconfig.toml> will generate a skeleton
    pub trait Region {
        // Declare methods that will be auto-generated
        // type use : insrcdata -trait xx.toml
        fn name(&self) -> &'static str;
        fn code(&self) -> u8;
        fn subregions(&self) -> db::SubregionIter;

        // you can then define your own methods
        fn in_eurasia(&self) -> bool {
            let code = self.code();
            code == 142 || code == 150 // asia or europe
        }
    }

    // This function will print some information from a country
    fn print_country(country: &db::Country) {
        //  countries may not have subregion(see ANTARTICA), so the field is optional
        match country.subregion() {
            Some(subregion) => {
                // the field region of subregion is not optional
                println!(
                    " {} ( ISO 3166‑1: {} ) in  {} from {}",
                    country.name(),
                    country.alpha3(),
                    subregion.name(),
                    subregion.region().name()
                );
            }
            None => {
                println!(
                    " {} ( ISO 3166‑1: {} ) has no subregion",
                    country.name(),
                    country.alpha3()
                );
            }
        }
    }

    pub fn demo() {
        // row access by label
        let belgium = db::Countries::Belgium;
        // we can access data members from the label
        println!("\n  infos for {} :", belgium.name());
        // but we ned to deref the label to pass the row reference to a function
        print_country(&belgium);

        println!("\n  info for a country without subregion");
        print_country(&db::Countries::Antarctica);

        println!("\n  all countries with  alpha3 code starting by 'F'");
        for country in db::Country::alpha3_range("F", "G") {
            print_country(country);
        }

        println!("\n  country with UNO code  136");
        for country in db::Country::code_range(136, 136) {
            print_country(country);
        }

        println!("\n  all countries with UNO code  in the range [100..112] ");
        for country in db::Country::code_range(100, 112) {
            print_country(country);
        }

        println!("\n  all countries in Western Europe");
        let west_europe = belgium.subregion().expect("Belgium has a subregion");
        for country in west_europe.countries() {
            print_country(country);
        }

        println!("\n  all subregions in  europe");
        let europe = west_europe.region();
        for subregion in europe.subregions() {
            println!("{} ( code : {} )", subregion.name(), subregion.code());
        }

        println!("\n  the first 5 countries of the table");
        for country in &db::Country::array()[0..5] {
            print_country(country);
        }

        println!()
    }

    // start of non regression tests
    // the code that follow this point is not intended to be used as sample
    // and may be more difficult to read (but should still be correct)

    //  lookup by code
    fn alpha3_country(code: &str) -> Option<&'static db::Country> {
        // TODO : generate accessor from insrcdata to avoid the stop bound check
        db::Country::alpha3_range(code, code).next()
    }

    fn test_sdn_sgp(start: &str, stop: &str) {
        let codes: Vec<String> = db::Country::alpha3_range(start, stop)
            .map(|n| n.alpha3().to_string())
            .collect();
        assert!(codes == vec!["SDN", "SEN", "SGP"]) //Sudan, Senegal, Singapore
    }

    pub fn test() {
        let belgium = db::Countries::Belgium;
        assert!(belgium.name() == "Belgium");
        assert!(belgium.alpha3() == "BEL");
        assert!(belgium.alpha2() == "BE");
        assert!(belgium.code() == 56);

        let west_europe = belgium.subregion().expect("Belgium has a subregion");
        let europe = west_europe.region();
        assert!(europe.in_eurasia());

        let benin = db::Country::alpha3_range("BEN", "ZZZ")
            .next()
            .expect("BEN -> Benin");
        assert!(benin.name() == "Benin");
        let subsahara = benin.subregion().expect("Benin has a subregion");
        assert!(subsahara.code() == 202);
        let africa = subsahara.region();
        assert!(africa.code() == 2);
        assert!(!africa.in_eurasia());

        // check reverse join 0..1
        let mut benin_in_subsahara = false;
        for country in subsahara.countries() {
            if country.code() == benin.code() {
                benin_in_subsahara = true;
            }
        }
        assert!(benin_in_subsahara);

        // check reverse join 1..1
        let mut subsahara_in_africa = false;
        for subregion in africa.subregions() {
            if subregion.code() == subsahara.code() {
                subsahara_in_africa = true;
            }
        }
        assert!(subsahara_in_africa);

        //  lookup by code
        let france = alpha3_country("FRA").expect("FRA -> France");
        assert!(france.name() == "France");
        let unknown = alpha3_country("XYZ");
        assert!(unknown.is_none());

        // test open and closed iter range
        test_sdn_sgp("SDN", "SGP");
        test_sdn_sgp("SDM", "SGP");
        test_sdn_sgp("SDN", "SGQ");
        test_sdn_sgp("SDM", "SGQ");
    }
} // mod country

fn main() {
    country::test();
    country::demo();
}
