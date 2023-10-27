// generated by insrcdata version 0.2.0

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ministers {
    DavidCameron = 0,
    GordonBrown = 1,
    RomanoProdi = 2,
}
impl From<Ministers> for  &'static Minister{
    fn from(value:Ministers) -> Self {
        &minister::TABLE[value as usize]
    }
}
impl From<&Ministers> for  &'static Minister{
    fn from(value: &Ministers) -> Self {
        &minister::TABLE[*value as usize]
    }
}
impl PartialEq<Ministers> for &Minister {
    fn eq(&self, other: &Ministers) -> bool {
        std::ptr::eq(<&Minister>::from(other), *self)
    }
}

pub struct Minister {
    name_ : &'static str,
    birth_ : u16,
    country_ : u8,
}
impl PartialEq<Self> for Minister {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Minister {}
impl std::hash::Hash for Minister {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        minister::index_of(self).hash(state);
    }
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
    pub fn country(&self) -> &'static Country { &country::TABLE[self.country_ as usize]}
}

mod minister {use super::*;

pub fn index_of(fic:&Minister) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Minister>()
}
pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static Minister;

    fn next(&mut self) -> Option<&'static Minister> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}


const fn r(name:&'static str, birth:u16, country:u8, ) -> Minister {
    Minister{name_:name, birth_:birth, country_:country, }
}

pub static TABLE : [ Minister ; 3 ] = [
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Countries {
    Uk = 0,
    It = 1,
}
impl From<Countries> for  &'static Country{
    fn from(value:Countries) -> Self {
        &country::TABLE[value as usize]
    }
}
impl From<&Countries> for  &'static Country{
    fn from(value: &Countries) -> Self {
        &country::TABLE[*value as usize]
    }
}
impl PartialEq<Countries> for &Country {
    fn eq(&self, other: &Countries) -> bool {
        std::ptr::eq(<&Country>::from(other), *self)
    }
}

pub struct Country {
    code_ : &'static str,
    name_ : &'static str,
}
impl PartialEq<Self> for Country {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Country {}
impl std::hash::Hash for Country {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        country::index_of(self).hash(state);
    }
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

mod country {use super::*;

pub fn index_of(fic:&Country) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Country>()
}
pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static Country;

    fn next(&mut self) -> Option<&'static Country> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}


const fn r(code:&'static str, name:&'static str, ) -> Country {
    Country{code_:code, name_:name, }
}

pub static TABLE : [ Country ; 2 ] = [
   {r("UK", "United Kingdom", )},
   {r("IT", "Italy", )},
];

} // mod country

pub use country::IndexIter as CountryIter;
