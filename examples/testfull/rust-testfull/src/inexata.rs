// generated by insrcdata version 0.1.0

#![allow(dead_code)]
#![allow(unused_variables)]
use std::ops::Deref;
#[derive(Clone, Copy)]
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
pub struct Person {
    name_ : &'static str,
    spouse_ : u8,
    father_ : u8,
    mother_ : u8,
}
impl Person {
    pub fn name(&self) -> &'static str { self.name_ }
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

const fn r(name:&'static str, spouse:u8, father:u8, mother:u8, ) -> super::Person {
    super::Person{name_:name, spouse_:spouse, father_:father, mother_:mother, }
}

pub static TABLE : [ super::Person ; 4 ] = [
   {r("Marie Curie", 1, 0, 0, )},
   {r("Pierre Curie", 0, 0, 0, )},
   {r("Irène Joliot-Curie", 3, 2, 1, )},
   {r("Frédéric Joliot-Curie", 2, 0, 0, )},
];


} // mod person 