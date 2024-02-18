pub struct Customer {
    name: String,
    logo: String, //url of a logo
}

impl Customer {
    pub fn new(name: String, logo: String) -> Customer {
        Customer {
            name,
            logo,
        }
    }
}
