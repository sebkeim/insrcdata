// generated by insrcdata version 0.2.0

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Clients {
    John = 0,
    Alix = 1,
    David = 2,
}
impl From<Clients> for  &'static Client{
    fn from(value:Clients) -> Self {
        &client::TABLE[value as usize]
    }
}
impl From<&Clients> for  &'static Client{
    fn from(value: &Clients) -> Self {
        &client::TABLE[*value as usize]
    }
}
impl PartialEq<Clients> for &Client {
    fn eq(&self, other: &Clients) -> bool {
        std::ptr::eq(<&Client>::from(other), *self)
    }
}

pub struct Client {
    name_ : &'static str,
}
impl PartialEq<Self> for Client {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Client {}
impl std::hash::Hash for Client {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        client::index_of(self).hash(state);
    }
}

impl Client {
    pub fn name(&self) -> &'static str { self.name_ }
    pub fn transactions(&self) -> TransactionIter {
        let cons = client::index_of(self) as u8;

        // bissect left
        let mut lo = 0;
        let mut hi = transaction::CLIENT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons > transaction::TABLE[transaction::CLIENT_INDEX[mid] as usize].client_ {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let start = lo;

        // bissect-right
        hi = transaction::CLIENT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons < transaction::TABLE[transaction::CLIENT_INDEX[mid] as usize].client_  {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        transaction::IndexIter {
            indexes: Box::new(transaction::CLIENT_INDEX[start..lo].iter()),
        }
    }
}

mod client {use super::*;

pub fn index_of(fic:&Client) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Client>()
}
pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static Client;

    fn next(&mut self) -> Option<&'static Client> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}


const fn r(name:&'static str, ) -> Client {
    Client{name_:name, }
}

pub static TABLE : [ Client ; 3 ] = [
   {r("John", )},
   {r("Alix", )},
   {r("David", )},
];

} // mod client

pub use client::IndexIter as ClientIter;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Products {
    Apple = 0,
    Banana = 1,
    Peach = 2,
    Cherry = 3,
}
impl From<Products> for  &'static Product{
    fn from(value:Products) -> Self {
        &product::TABLE[value as usize]
    }
}
impl From<&Products> for  &'static Product{
    fn from(value: &Products) -> Self {
        &product::TABLE[*value as usize]
    }
}
impl PartialEq<Products> for &Product {
    fn eq(&self, other: &Products) -> bool {
        std::ptr::eq(<&Product>::from(other), *self)
    }
}

pub struct Product {
    name_ : &'static str,
}
impl PartialEq<Self> for Product {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Product {}
impl std::hash::Hash for Product {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        product::index_of(self).hash(state);
    }
}

impl Product {
    pub fn name(&self) -> &'static str { self.name_ }
    pub fn transactions(&self) -> TransactionIter {
        let cons = product::index_of(self) as u8;

        // bissect left
        let mut lo = 0;
        let mut hi = transaction::PRODUCT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons > transaction::TABLE[transaction::PRODUCT_INDEX[mid] as usize].product_ {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let start = lo;

        // bissect-right
        hi = transaction::PRODUCT_INDEX.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if cons < transaction::TABLE[transaction::PRODUCT_INDEX[mid] as usize].product_  {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        transaction::IndexIter {
            indexes: Box::new(transaction::PRODUCT_INDEX[start..lo].iter()),
        }
    }
}

mod product {use super::*;

pub fn index_of(fic:&Product) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Product>()
}
pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static Product;

    fn next(&mut self) -> Option<&'static Product> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}


const fn r(name:&'static str, ) -> Product {
    Product{name_:name, }
}

pub static TABLE : [ Product ; 4 ] = [
   {r("Apple", )},
   {r("Banana", )},
   {r("Peach", )},
   {r("Cherry", )},
];

} // mod product

pub use product::IndexIter as ProductIter;
pub struct Transaction {
    client_ : u8,
    product_ : u8,
}
impl PartialEq<Self> for Transaction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for Transaction {}
impl std::hash::Hash for Transaction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        transaction::index_of(self).hash(state);
    }
}

impl Transaction {
    pub fn client(&self) -> &'static Client { &client::TABLE[self.client_ as usize]}
    pub fn product(&self) -> &'static Product { &product::TABLE[self.product_ as usize]}
}

mod transaction {use super::*;

pub fn index_of(fic:&Transaction) -> usize {
    ((fic  as *const _ as usize) - (&TABLE[0]  as *const _ as usize)) / std::mem::size_of::<Transaction>()
}
pub struct IndexIter {
    pub indexes : Box<dyn Iterator<Item=&'static u8>>,
}

impl Iterator for IndexIter {
    type Item = & 'static Transaction;

    fn next(&mut self) -> Option<&'static Transaction> {
        let idx = self.indexes.next();
        match idx {
            Some(v) => Some(&TABLE[*v as usize]),
            None => None,
        }
    }
}


const fn r(client:u8, product:u8, ) -> Transaction {
    Transaction{client_:client, product_:product, }
}

pub static TABLE : [ Transaction ; 7 ] = [
   {r(0, 0, )},
   {r(0, 1, )},
   {r(1, 0, )},
   {r(1, 2, )},
   {r(2, 0, )},
   {r(2, 1, )},
   {r(2, 2, )},
];
pub static CLIENT_INDEX : [ u8 ; 7 ] = [
    0, 1, 2, 3, 4, 5, 6, 
];
pub static PRODUCT_INDEX : [ u8 ; 7 ] = [
    0, 2, 4, 1, 5, 3, 6, 
];

} // mod transaction

pub use transaction::IndexIter as TransactionIter;
