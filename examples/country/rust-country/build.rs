// this build script generate insrcdata code from embedded table definition

fn main() -> insrcdata::Result<()> {
    insrcdata::build(
        r#"
help = "Countries and regions defined by UN M49 and  ISO 3166"

# TABLE : Region
    [[table]]
src = "../insrcdata/countries.csv"
name = 'Region'
help = "Continent of provenance"

    [[table.col]]
name = 'name'
src = 'region'
help = "English short name"

    [[table.col]]
name = 'code'
format  = 'u8'
src = 'region-code'
# we keep only one record by region
single = true
help = "UN M49 code"

# TABLE : Subregions
    [[table]]
src = "../insrcdata/countries.csv"
name = 'Subregion'
help = "Subcontinent as defined in UN M49"

[[table.col]]
name = 'name'
src = 'sub-region'
help = "English short name"

[[table.col]]
name = 'code'
format  = 'u16'
src = 'sub-region-code'
single = true
help = "UN M49 code"

[[table.join]]
name = 'region'
src = 'region-code'
to = 'region-code'
external = 'Region'
reverse = 'subregions'
help = "The region where the subregion is located"
reverse_help = "All the subregions belonging to the region"


   # TABLE : Country
[[table]]
src = "../insrcdata/countries.csv"
name = 'Country'
array = true
help = "Country according to ISO/UN"

[[table.col]]
name = 'name'
help = "English short name"

[[table.col]]
name = 'alpha2'
src = 'alpha-2'
help = "Two-letter country codes defined in ISO 3166-1"

[[table.col]]
name = 'alpha3'
src = 'alpha-3'
range = true
help = "Three-letter country codes defined in ISO 3166-1"
range_help = "Search country by ISO 3166-1 three-letter code"

[[table.col]]
name = 'code'
format  = 'u16'
src = 'country-code'
range = true
help = "UN M49 code"
range_help = "Search country by UN M49 code"

[[table.join]]
name = 'subregion'
src = 'sub-region-code'
to = 'sub-region-code'
external = 'Subregion'
optional = true
reverse = 'countries'
help = "Subregion containing the country"
reverse_help = "All the countries belonging to the subregions"

[[table.col]]
name = 'Countries'
src = 'label'
format = 'label'
help = "Identifiers for countries according to ISO/UN"
"#,
    )
}
