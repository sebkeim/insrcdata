// this build script generate insrcdata code from embedded table definition

fn main() -> insrcdata::Result<()> {
    insrcdata::build(r#"

    # TABLE : Region
        [[table]]
    src = "../insrcdata/countries.csv"
    name = 'Region'

        [[table.col]]
    name = 'name'
    src = 'region'

        [[table.col]]
    name = 'code'
    format  = 'u8'
    src = 'region-code'
    # we keep only one record by region
    single = true

    # TABLE : Subregions
        [[table]]
    src = "../insrcdata/countries.csv"
    name = 'Subregion'

        [[table.col]]
    name = 'name'
    src = 'sub-region'

        [[table.col]]
    name = 'code'
    format  = 'u16'
    src = 'sub-region-code'
    single = true

        [[table.join]]
    name = 'region'
    src = 'region-code'
    to = 'region-code'
    external = 'Region'
    reverse = 'subregions'

    # TABLE : Country
        [[table]]
    src = "../insrcdata/countries.csv"
    name = 'Country'
    array = true

        [[table.col]]
    name = 'name'

        [[table.col]]
    name = 'alpha2'
    src = 'alpha-2'

        [[table.col]]
    name = 'alpha3'
    src = 'alpha-3'
    range = true

        [[table.col]]
    name = 'code'
    format  = 'u16'
    src = 'country-code'
    range = true

        [[table.join]]
    name = 'subregion'
    src = 'sub-region-code'
    to = 'sub-region-code'
    external = 'Subregion'
    optional = true
    reverse = 'countries'

        [[table.col]]
    name = 'Countries'
    src = 'label'
    format = 'label'
"#)
}
