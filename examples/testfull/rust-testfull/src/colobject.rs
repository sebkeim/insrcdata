use crate::insrcdata as db;

// object type column are used to
pub fn make_upper(s:&str)->String{
    s.to_uppercase()
}
pub fn make_lower(s:&str)->String{
    s.to_uppercase()
}
pub fn make_capitalize(_s:&str)->String{
    "TODO".to_string()
}

pub fn test_colobject() {
    let upper = db::Lettercases::Upper;
    assert!(upper.transformer()("hello")== "HELLO");
}
