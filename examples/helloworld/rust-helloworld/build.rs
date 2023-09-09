// build script using the hight level API

fn main() -> insrcdata::Result<()> {
    insrcdata::read("../insrcdata/insrcdata.toml")
}
