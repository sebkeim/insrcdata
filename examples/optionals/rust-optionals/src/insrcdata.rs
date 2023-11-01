// generated by insrcdata version 0.2.0

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Adhocs {
    Filled = 0,
    Empty = 1,
}
impl From<Adhocs> for  &'static Adhoc{
    fn from(value:Adhocs) -> Self {
        &adhoc::TABLE[value as usize]
    }
}
impl From<&Adhocs> for  &'static Adhoc{
    fn from(value: &Adhocs) -> Self {
        &adhoc::TABLE[*value as usize]
    }
}
impl PartialEq<Adhocs> for &Adhoc {
    fn eq(&self, other: &Adhocs) -> bool {
        std::ptr::eq(<&Adhoc>::from(other), *self)
    }
}

pub struct Adhoc {
    score_data_ : f32,
    count_data_ : u8,
}
impl PartialEq<Self> for Adhoc {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Adhoc {}
impl std::hash::Hash for Adhoc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        adhoc::index_of(self).hash(state);
    }
}

impl Adhoc {
    pub fn score_data(&self) -> f32 { self.score_data_ }
    pub fn count_data(&self) -> u16 { self.count_data_ as u16 }
}

mod adhoc {use super::*;

pub fn index_of(fic:&Adhoc) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Adhoc>()
}
const fn r(score_data:f32, count_data:u8, ) -> Adhoc {
    Adhoc{score_data_:score_data, count_data_:count_data, }
}

pub static TABLE : [ Adhoc ; 2 ] = [
   {r(100.0, 42, )},
   {r(-1.0, 0, )},
];

} // mod adhoc

pub struct Score {
    value_ : f32,
}
impl PartialEq<Self> for Score {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Score {}
impl std::hash::Hash for Score {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        score::index_of(self).hash(state);
    }
}

impl Score {
    pub fn value(&self) -> f32 { self.value_ }
}

mod score {use super::*;

pub fn index_of(fic:&Score) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Score>()
}
const fn r(value:f32, ) -> Score {
    Score{value_:value, }
}

pub static TABLE : [ Score ; 1 ] = [
   {r(100.0, )},
];

} // mod score

pub struct Count {
    value_ : u8,
}
impl PartialEq<Self> for Count {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Count {}
impl std::hash::Hash for Count {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        count::index_of(self).hash(state);
    }
}

impl Count {
    pub fn value(&self) -> u16 { self.value_ as u16 }
}

mod count {use super::*;

pub fn index_of(fic:&Count) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Count>()
}
const fn r(value:u8, ) -> Count {
    Count{value_:value, }
}

pub static TABLE : [ Count ; 1 ] = [
   {r(42, )},
];

} // mod count

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Optjoins {
    Filled = 0,
    Empty = 1,
}
impl From<Optjoins> for  &'static Optjoin{
    fn from(value:Optjoins) -> Self {
        &optjoin::TABLE[value as usize]
    }
}
impl From<&Optjoins> for  &'static Optjoin{
    fn from(value: &Optjoins) -> Self {
        &optjoin::TABLE[*value as usize]
    }
}
impl PartialEq<Optjoins> for &Optjoin {
    fn eq(&self, other: &Optjoins) -> bool {
        std::ptr::eq(<&Optjoin>::from(other), *self)
    }
}

pub struct Optjoin {
    score_join_ : u8,
    count_join_ : u8,
}
impl PartialEq<Self> for Optjoin {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Optjoin {}
impl std::hash::Hash for Optjoin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        optjoin::index_of(self).hash(state);
    }
}

impl Optjoin {
    pub fn score_join(&self) -> Option<&'static Score> {
        let index = self.score_join_;
        if index==0 { None } else { Some(&score::TABLE[index as usize -1]) }
    }
    pub fn count_join(&self) -> Option<&'static Count> {
        let index = self.count_join_;
        if index==0 { None } else { Some(&count::TABLE[index as usize -1]) }
    }
}

mod optjoin {use super::*;

pub fn index_of(fic:&Optjoin) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Optjoin>()
}
const fn r(score_join:u8, count_join:u8, ) -> Optjoin {
    Optjoin{score_join_:score_join, count_join_:count_join, }
}

pub static TABLE : [ Optjoin ; 2 ] = [
   {r(1, 1, )},
   {r(0, 0, )},
];

} // mod optjoin

