use crate::insrcdata as db;

// object type column are used to link to rust methods
pub fn make_upper(s: &str) -> String {
    s.to_uppercase()
}
pub fn make_lower(s: &str) -> String {
    s.to_uppercase()
}
pub fn make_capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_ascii_uppercase().to_string() + c.as_str(),
    }
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}
pub static ZERO: Point = Point { x: 0.0, y: 0.0 };
pub static ONE: Point = Point { x: 1.0, y: 1.0 };

pub fn test_colobject() {
    let upper: &db::Lettercase = db::Lettercases::Upper.into();
    assert!(upper.transformer()("hello") == "HELLO");
    assert!(upper.point().x == 1.0);

    let capital: &db::Lettercase = db::Lettercases::Capital.into();
    assert!(capital.transformer()("hello") == "Hello");
    assert!(capital.point().x == 0.0);
}
