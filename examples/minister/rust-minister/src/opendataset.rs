/// Open dataset is a conceptual model of data that allow the client to extend the number of variant
/// values provided by a library.
///
/// This sample extend the predefined Minister table defined in insrcdata with custom elements

/// in the common case  we know in advance how to implement custom elements
/// the best option is then to use an enum   
pub mod inapp {
    use crate::insrcdata;

    /// define shared behaviors between predefined and custom elements
    enum Minister {
        /// Predefined elements handle to insrcdata record
        Predefined(&'static insrcdata::Minister),

        /// Custom elements own their data
        /// In a more realistic case, the enum could store a box reference to data
        Custom { name: String, birth: u16 },
    }

    /// methods shared between predefined and custom elements

    impl Minister {
        fn name(&self) -> &str {
            match self {
                Minister::Predefined(fic) => fic.name(),
                Minister::Custom { name, .. } => name,
            }
        }
        fn birth(&self) -> u16 {
            match self {
                Minister::Predefined(fic) => fic.birth(),
                Minister::Custom { birth, .. } => *birth,
            }
        }
    }

    // sample of a function that consume the enum
    fn minister_info(minister: &Minister) {
        // access it's attributes
        println!(
            "Minister {} was born in {}.",
            minister.name(),
            minister.birth()
        );

        if let Minister::Predefined(fic) = minister {
            println!("  country : {}", fic.country().name());
        }
    }

    // usage
    pub fn sample() {
        // using predefinded record
        let r_prodi = Minister::Predefined(insrcdata::Ministers::RomanoProdi.into());
        minister_info(&r_prodi);

        // using custom object
        let a_merkel = Minister::Custom {
            name: "Angela Merkel".to_string(),
            birth: 1954,
        };
        minister_info(&a_merkel);
    }
}

// =================================================================================================

/// library authors may not know how client will implement custom elements
/// in that case we have to use a trait
pub mod inlib {
    use crate::insrcdata;

    // This part would belong to the library source code

    /// define shared behaviors between predefined and custom elements
    trait Ministrable {
        /// actual insrcdata record managing the trait, return None for custom elements
        //  todo: use TryForm trait instead of as_predefined ?
        fn as_predefined(&self) -> Option<&insrcdata::Minister> {
            None
        }

        // sample of  shared behaviors between predefined and custom elements
        fn name(&self) -> &str;
        fn birth(&self) -> u16;
    }

    /// shared behaviors implementation for insrcdata record
    impl Ministrable for insrcdata::Minister {
        fn as_predefined(&self) -> Option<&insrcdata::Minister> {
            Some(self)
        }

        fn name(&self) -> &str {
            <insrcdata::Minister>::name(self)
        }
        fn birth(&self) -> u16 {
            <insrcdata::Minister>::birth(self)
        }
    }

    // sample of a function that consume the Trait
    fn ministrable_info(mnstr: &dyn Ministrable) {
        // access it's attributes
        println!("Minister {} was born in {}.", mnstr.name(), mnstr.birth());

        if let Some(f) = mnstr.as_predefined() {
            println!("  country : {}", f.country().name());
        }
    }

    // The following would belong to client source code

    /// Custom element
    struct CustomMinister {
        name: String,
        birth: u16,
    }

    impl Ministrable for CustomMinister {
        fn name(&self) -> &str {
            self.name.as_str()
        }
        fn birth(&self) -> u16 {
            self.birth
        }
    }

    // usage
    pub fn sample() {
        // using predefinded record
        let r_prodi: &insrcdata::Minister = insrcdata::Ministers::DavidCameron.into();
        ministrable_info(r_prodi);

        // using custom object
        let a_merkel = CustomMinister {
            name: "José Luis Rodríguez Zapatero".to_string(),
            birth: 1960,
        };
        ministrable_info(&a_merkel);
    }
}

// todo: show how to do search between both predefined and custom elements :
//  - a lookup method that return from an identifier ( for sample wikidata QID of the minister )
//  - implement birth_range search  ?
