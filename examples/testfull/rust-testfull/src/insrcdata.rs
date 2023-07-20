// generated by insrcdata version 0.1.0

#![allow(dead_code)]
#![allow(unused_variables)]
use std::ops::Deref;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Persons {
    Marie = 0,
    Pierre = 1,
    Irene = 2,
    Frederic = 3,
}
impl Deref for Persons {
    type Target =  Person;
    fn deref(&self) -> &'static Person {
        &person::TABLE[*self as usize]
    }
}
impl PartialEq<&Person> for Persons {
    fn eq(&self, other: &&Person) -> bool {
        std::ptr::eq(self as &Person, *other)
    }
}

pub struct Person {
    name_ : &'static str,
    woman_ : bool,
    score_ : f64,
    spouse_ : u8,
    father_ : u8,
    mother_ : u8,
}
impl PartialEq<Self> for Person {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Person {
    pub fn name(&self) -> &'static str { self.name_ }
    pub fn woman(&self) -> bool { self.woman_ }
    pub fn score(&self) -> f64 { self.score_ }

    pub fn score_range(start:f64, stop:f64) -> person::IndexIter {
        let mut lo = 0;
        let mut hi = person::SCORE_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if start > person::TABLE[person::SCORE_INDEX[mid] as usize].score_ {
                 lo = mid + 1;
            } else {
                 hi = mid;
            }
        }

        let begin = lo;
        hi = person::SCORE_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if stop < person::TABLE[person::SCORE_INDEX[mid] as usize].score_ {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        person::IndexIter {
            indexes: Box::new(person::SCORE_INDEX[begin..lo].iter()),
        }
    }
    pub fn spouse(&self) -> &'static Person {
        &person::TABLE[self.spouse_ as usize]
    }
    pub fn father(&self) -> Option<&'static Person> {
        let index = self.father_;
        if index==0 { None } else { Some(&person::TABLE[index as usize -1]) }
    }
    pub fn mother(&self) -> Option<&'static Person> {
        let index = self.mother_;
        if index==0 { None } else { Some(&person::TABLE[index as usize -1]) }
    }
}

mod person {


use std::mem;

pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static super::Person;

    fn next(&mut self) -> Option<&'static super::Person> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}

pub fn index_of(fic:&super::Person) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / mem::size_of::<super::Person>()
}

const fn r(name:&'static str, woman:bool, score:f64, spouse:u8, father:u8, mother:u8, ) -> super::Person {
    super::Person{name_:name, woman_:woman, score_:score, spouse_:spouse, father_:father, mother_:mother, }
}

pub static TABLE : [ super::Person ; 4 ] = [
   {r("Marie Curie", true, 1.0, 1, 0, 0, )},
   {r("Pierre Curie", false, 2.1, 0, 0, 0, )},
   {r("Irène Joliot-Curie", true, 3.2, 3, 2, 1, )},
   {r("Frédéric Joliot-Curie", false, 4.3, 2, 0, 0, )},
];
pub static SCORE_INDEX : [ u8 ; 4 ] = [
    0, 1, 2, 3, 
];

} // mod person

pub use person::IndexIter as PersonIter;
pub struct Strencoding {
    text_ : &'static str,
}
impl PartialEq<Self> for Strencoding {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Strencoding {
    pub fn text(&self) -> &'static str { self.text_ }

    pub fn text_range(start:& str, stop:& str) -> strencoding::IndexIter {
        let mut lo = 0;
        let mut hi = strencoding::TEXT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if start > strencoding::TABLE[strencoding::TEXT_INDEX[mid] as usize].text_ {
                 lo = mid + 1;
            } else {
                 hi = mid;
            }
        }

        let begin = lo;
        hi = strencoding::TEXT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if stop < strencoding::TABLE[strencoding::TEXT_INDEX[mid] as usize].text_ {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        strencoding::IndexIter {
            indexes: Box::new(strencoding::TEXT_INDEX[begin..lo].iter()),
        }
    }
    pub fn array() -> &'static [Strencoding; 6]  { &strencoding::TABLE }
}

mod strencoding {


use std::mem;

pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static super::Strencoding;

    fn next(&mut self) -> Option<&'static super::Strencoding> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}

pub fn index_of(fic:&super::Strencoding) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / mem::size_of::<super::Strencoding>()
}

const fn r(text:&'static str, ) -> super::Strencoding {
    super::Strencoding{text_:text, }
}

pub static TABLE : [ super::Strencoding ; 6 ] = [
   {r("𝒾ň𝗌яčḓẚᵵᶏ : 𝔢ᶆḃ℮𝚍 ᶌ𝖔ừᵳ ⅆằƫⱥ", )},
   {r("hello", )},
   {r("κόσμε", )},
   {r("いろはにほへとちりぬるを", )},
   {r("éventuellement validé", )},
   {r("Да, но фальшивый экземпляр", )},
];
pub static TEXT_INDEX : [ u8 ; 6 ] = [
    1, 4, 2, 5, 3, 0, 
];

} // mod strencoding

pub use strencoding::IndexIter as StrencodingIter;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Lettercases {
    Capital = 0,
    Upper = 1,
    Lower = 2,
}
impl Deref for Lettercases {
    type Target =  Lettercase;
    fn deref(&self) -> &'static Lettercase {
        &lettercase::TABLE[*self as usize]
    }
}
impl PartialEq<&Lettercase> for Lettercases {
    fn eq(&self, other: &&Lettercase) -> bool {
        std::ptr::eq(self as &Lettercase, *other)
    }
}

pub struct Lettercase {
    name_ : &'static str,
    transformer_ : fn(&str)->String,
    point_ : &'static crate::colobject::Point,
}
impl PartialEq<Self> for Lettercase {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Lettercase {
    pub fn name(&self) -> &'static str { self.name_ }
    pub fn transformer(&self) -> fn(&str)->String { self.transformer_ }
    pub fn point(&self) -> &'static crate::colobject::Point { self.point_ }
}

mod lettercase {
use crate::colobject as co;

const fn r(name:&'static str, transformer:fn(&str)->String, point:&'static crate::colobject::Point, ) -> super::Lettercase {
    super::Lettercase{name_:name, transformer_:transformer, point_:point, }
}

pub static TABLE : [ super::Lettercase ; 3 ] = [
   {r("Capitalised case", co::make_capitalize, &co::ZERO, )},
   {r("Upper case", co::make_upper, &co::ONE, )},
   {r("Lower case", co::make_lower, &co::ONE, )},
];

} // mod lettercase

