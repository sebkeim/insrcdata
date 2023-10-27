// generated by insrcdata version 0.2.0

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Persons {
    Marie = 0,
    Pierre = 1,
    Irene = 2,
    Frederic = 3,
}
impl From<Persons> for  &'static Person{
    fn from(value:Persons) -> Self {
        &person::TABLE[value as usize]
    }
}
impl From<&Persons> for  &'static Person{
    fn from(value: &Persons) -> Self {
        &person::TABLE[*value as usize]
    }
}
impl PartialEq<Persons> for &Person {
    fn eq(&self, other: &Persons) -> bool {
        std::ptr::eq(<&Person>::from(other), *self)
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
impl Eq for Person {}
impl std::hash::Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        person::index_of(self).hash(state);
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
    pub fn spouse(&self) -> &'static Person { &person::TABLE[self.spouse_ as usize]}
    pub fn father(&self) -> Option<&'static Person> {
        let index = self.father_;
        if index==0 { None } else { Some(&person::TABLE[index as usize -1]) }
    }
    pub fn mother(&self) -> Option<&'static Person> {
        let index = self.mother_;
        if index==0 { None } else { Some(&person::TABLE[index as usize -1]) }
    }

    pub fn wdata(&self) -> WikidataIter {
        let cons = person::index_of(self) as u8;

        // bissect left
        let mut lo = 0;
        let mut hi = wikidata::OBJECT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons > wikidata::TABLE[wikidata::OBJECT_INDEX[mid] as usize].object_ {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let start = lo;

        // bissect-right
        hi = wikidata::OBJECT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons < wikidata::TABLE[wikidata::OBJECT_INDEX[mid] as usize].object_  {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        wikidata::IndexIter {
            indexes: Box::new(wikidata::OBJECT_INDEX[start..lo].iter()),
        }
    }

    pub fn congress(&self) -> CongressIter {
        let cons = person::index_of(self) as u8 + 1;

        // bissect left
        let mut lo = 0;
        let mut hi = congress::OBJECT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons > congress::TABLE[congress::OBJECT_INDEX[mid] as usize].object_ {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let start = lo;

        // bissect-right
        hi = congress::OBJECT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons < congress::TABLE[congress::OBJECT_INDEX[mid] as usize].object_  {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        congress::IndexIter {
            indexes: Box::new(congress::OBJECT_INDEX[start..lo].iter()),
        }
    }
}

mod person {use super::*;

pub fn index_of(fic:&Person) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Person>()
}
pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static Person;

    fn next(&mut self) -> Option<&'static Person> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}


const fn r(name:&'static str, woman:bool, score:f64, spouse:u8, father:u8, mother:u8, ) -> Person {
    Person{name_:name, woman_:woman, score_:score, spouse_:spouse, father_:father, mother_:mother, }
}

pub static TABLE : [ Person ; 4 ] = [
   {r("Marie Curie", true, 1.0, 1, 0, 0, )},
   {r("Pierre Curie", false, 2.1, 0, 0, 0, )},
   {r("Irène Joliot-Curie", true, 3.2, 3, 2, 1, )},
   {r("Frédéric Joliot-Curie", false, 2.1, 2, 0, 0, )},
];
pub static SCORE_INDEX : [ u8 ; 4 ] = [
    0, 1, 3, 2, 
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
impl Eq for Strencoding {}
impl std::hash::Hash for Strencoding {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        strencoding::index_of(self).hash(state);
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
    pub fn array() -> &'static [Strencoding; 6] { &strencoding::TABLE }
    pub fn as_index(&self) -> usize { strencoding::index_of(self) }
}

mod strencoding {use super::*;

pub fn index_of(fic:&Strencoding) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Strencoding>()
}
pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static Strencoding;

    fn next(&mut self) -> Option<&'static Strencoding> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}


const fn r(text:&'static str, ) -> Strencoding {
    Strencoding{text_:text, }
}

pub static TABLE : [ Strencoding ; 6 ] = [
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
impl From<Lettercases> for  &'static Lettercase{
    fn from(value:Lettercases) -> Self {
        &lettercase::TABLE[value as usize]
    }
}
impl From<&Lettercases> for  &'static Lettercase{
    fn from(value: &Lettercases) -> Self {
        &lettercase::TABLE[*value as usize]
    }
}
impl PartialEq<Lettercases> for &Lettercase {
    fn eq(&self, other: &Lettercases) -> bool {
        std::ptr::eq(<&Lettercase>::from(other), *self)
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
impl Eq for Lettercase {}
impl std::hash::Hash for Lettercase {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        lettercase::index_of(self).hash(state);
    }
}

impl Lettercase {
    pub fn name(&self) -> &'static str { self.name_ }
    pub fn transformer(&self) -> fn(&str)->String { self.transformer_ }
    pub fn point(&self) -> &'static crate::colobject::Point { self.point_ }

    pub fn wdata2(&self) -> WikidataIter {
        let cons = lettercase::index_of(self) as u8 + 4;

        // bissect left
        let mut lo = 0;
        let mut hi = wikidata::OBJECT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons > wikidata::TABLE[wikidata::OBJECT_INDEX[mid] as usize].object_ {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let start = lo;

        // bissect-right
        hi = wikidata::OBJECT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons < wikidata::TABLE[wikidata::OBJECT_INDEX[mid] as usize].object_  {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        wikidata::IndexIter {
            indexes: Box::new(wikidata::OBJECT_INDEX[start..lo].iter()),
        }
    }

    pub fn congress(&self) -> CongressIter {
        let cons = lettercase::index_of(self) as u8 + 5;

        // bissect left
        let mut lo = 0;
        let mut hi = congress::OBJECT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons > congress::TABLE[congress::OBJECT_INDEX[mid] as usize].object_ {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let start = lo;

        // bissect-right
        hi = congress::OBJECT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons < congress::TABLE[congress::OBJECT_INDEX[mid] as usize].object_  {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        congress::IndexIter {
            indexes: Box::new(congress::OBJECT_INDEX[start..lo].iter()),
        }
    }
}

mod lettercase {use super::*;
use crate::colobject as co;

pub fn index_of(fic:&Lettercase) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Lettercase>()
}
pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static Lettercase;

    fn next(&mut self) -> Option<&'static Lettercase> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}


const fn r(name:&'static str, transformer:fn(&str)->String, point:&'static crate::colobject::Point, ) -> Lettercase {
    Lettercase{name_:name, transformer_:transformer, point_:point, }
}

pub static TABLE : [ Lettercase ; 3 ] = [
   {r("Capitalised case", co::make_capitalize, &co::ZERO, )},
   {r("Upper case", co::make_upper, &co::ONE, )},
   {r("Lower case", co::make_lower, &co::ONE, )},
];

} // mod lettercase

pub use lettercase::IndexIter as LettercaseIter;
pub struct Wikidata {
    qid_ : u32,
    object_ : u8,
}
impl PartialEq<Self> for Wikidata {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Wikidata {}
impl std::hash::Hash for Wikidata {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        wikidata::index_of(self).hash(state);
    }
}

impl Wikidata {
    pub fn qid(&self) -> u32 { self.qid_ }
    pub fn object(&self) -> WikidataObject { 
        let v = self.object_ ;
        match v {
             0..=3 => WikidataObject::Person(&person::TABLE[v as usize ]),
             4..=6 => WikidataObject::Lettercase(&lettercase::TABLE[v as usize  - 4]),
             _ => panic!("insrcdata variant index overflow"),
        }
    }
    pub fn array() -> &'static [Wikidata; 3] { &wikidata::TABLE }
    pub fn as_index(&self) -> usize { wikidata::index_of(self) }
}

mod wikidata {use super::*;

pub fn index_of(fic:&Wikidata) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Wikidata>()
}
pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static Wikidata;

    fn next(&mut self) -> Option<&'static Wikidata> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}


const fn r(qid:u32, object:u8, ) -> Wikidata {
    Wikidata{qid_:qid, object_:object, }
}

pub static TABLE : [ Wikidata ; 3 ] = [
   {r(7186, 0, )},
   {r(8185162, 6, )},
   {r(150989, 3, )},
];
pub static OBJECT_INDEX : [ u8 ; 3 ] = [
    0, 2, 1, 
];

} // mod wikidata

pub use wikidata::IndexIter as WikidataIter;
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum  WikidataObject {
     Person(&'static Person),
     Lettercase(&'static Lettercase),
}

pub struct Congress {
    lccn_ : &'static str,
    object_ : u8,
}
impl PartialEq<Self> for Congress {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Congress {}
impl std::hash::Hash for Congress {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        congress::index_of(self).hash(state);
    }
}

impl Congress {
    pub fn lccn(&self) -> &'static str { self.lccn_ }
    pub fn object(&self) -> CongressObject { 
        let v = self.object_ ;
        match v {
             0..=0 => CongressObject::None,
             1..=4 => CongressObject::Person(&person::TABLE[v as usize  - 1]),
             5..=7 => CongressObject::Lettercase(&lettercase::TABLE[v as usize  - 5]),
             _ => panic!("insrcdata variant index overflow"),
        }
    }
    pub fn array() -> &'static [Congress; 4] { &congress::TABLE }
    pub fn as_index(&self) -> usize { congress::index_of(self) }
}

mod congress {use super::*;

pub fn index_of(fic:&Congress) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Congress>()
}
pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static Congress;

    fn next(&mut self) -> Option<&'static Congress> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}


const fn r(lccn:&'static str, object:u8, ) -> Congress {
    Congress{lccn_:lccn, object_:object, }
}

pub static TABLE : [ Congress ; 4 ] = [
   {r("n2009011553", 1, )},
   {r("sh85148650", 7, )},
   {r("n80159913", 4, )},
   {r("n79006404", 0, )},
];
pub static OBJECT_INDEX : [ u8 ; 3 ] = [
    0, 2, 1, 
];

} // mod congress

pub use congress::IndexIter as CongressIter;
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum  CongressObject {
     None,
     Person(&'static Person),
     Lettercase(&'static Lettercase),
}

