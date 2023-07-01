// generated by insrcdata version 0.1.0

#![allow(dead_code)]
#![allow(unused_variables)]
use std::ops::Deref;
#[derive(Clone, Copy)]
pub enum Ministers {
    DavidCameron = 0,
    GordonBrown = 1,
    RomanoProdi = 2,
}
impl Deref for Ministers {
    type Target =  Minister;
    fn deref(&self) -> &'static Minister {
        &minister::TABLE[*self as usize]
    }
}
pub struct Minister {
    name_ : &'static str,
    birth_ : u16,
    country_ : u8,
}
impl Minister {
    pub fn name(&self) -> &'static str { self.name_ }
    pub fn birth(&self) -> u16 { self.birth_ }

    pub fn birth_range(start:u16, stop:u16) -> minister::IndexIter {
        let mut lo = 0;
        let mut hi = minister::BIRTH_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if start > minister::TABLE[minister::BIRTH_INDEX[mid] as usize].birth_ {
                 lo = mid + 1;
            } else {
                 hi = mid;
            }
        }

        let begin = lo;
        hi = minister::BIRTH_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if stop < minister::TABLE[minister::BIRTH_INDEX[mid] as usize].birth_ {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        minister::IndexIter {
            indexes: Box::new(minister::BIRTH_INDEX[begin..lo].iter()),
        }
    }
    pub fn country(&self) -> &'static Country {
        &country::TABLE[self.country_ as usize]
    }
}

mod minister {


use std::mem;

pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static super::Minister;

    fn next(&mut self) -> Option<&'static super::Minister> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}

pub fn index_of(fic:&super::Minister) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / mem::size_of::<super::Minister>()
}

const fn r(name:&'static str, birth:u16, country:u8, ) -> super::Minister {
    super::Minister{name_:name, birth_:birth, country_:country, }
}

pub static TABLE : [ super::Minister ; 3 ] = [
   {r("David Cameron", 1966, 0, )},
   {r("Gordon Brown", 1951, 0, )},
   {r("Romano Prodi", 1939, 1, )},
];
pub static BIRTH_INDEX : [ u8 ; 3 ] = [
    2, 1, 0, 
];
pub static COUNTRY_INDEX : [ u8 ; 3 ] = [
    0, 1, 2, 
];

} // mod minister

pub use minister::IndexIter as MinisterIter;
#[derive(Clone, Copy)]
pub enum Countries {
    Gb = 0,
    It = 1,
}
impl Deref for Countries {
    type Target =  Country;
    fn deref(&self) -> &'static Country {
        &country::TABLE[*self as usize]
    }
}
pub struct Country {
    code_ : &'static str,
    name_ : &'static str,
}
impl Country {
    pub fn code(&self) -> &'static str { self.code_ }
    pub fn name(&self) -> &'static str { self.name_ }

    pub fn ministers(&self) -> MinisterIter {
        let cons = country::index_of(self) as u8;

        // bissect left
        let mut lo = 0;
        let mut hi = minister::COUNTRY_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons > minister::TABLE[minister::COUNTRY_INDEX[mid] as usize].country_ {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let start = lo;

        // bissect-right
        hi = minister::COUNTRY_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons < minister::TABLE[minister::COUNTRY_INDEX[mid] as usize].country_  {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        minister::IndexIter {
            indexes: Box::new(minister::COUNTRY_INDEX[start..lo].iter()),
        }
    }
}

mod country {


use std::mem;

pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static super::Country;

    fn next(&mut self) -> Option<&'static super::Country> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}

pub fn index_of(fic:&super::Country) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / mem::size_of::<super::Country>()
}

const fn r(code:&'static str, name:&'static str, ) -> super::Country {
    super::Country{code_:code, name_:name, }
}

pub static TABLE : [ super::Country ; 2 ] = [
   {r("GB", "United Kingdom", )},
   {r("IT", "Italy", )},
];

} // mod country

pub use country::IndexIter as CountryIter;
