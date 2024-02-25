mod testimonial;
mod customer;

use crate::testimonial::{Consent, Testimonial, Source};
use customer as cs;

fn main() {

    let mut customer1 = cs::Customer::new(
        String::from("John"), 
        String::from("url1")
    );

    let mut testimonial1 = Testimonial::new(
        customer1.name.clone(),
        String::from("Great product!"),
        String::from("2024-02-16"),
        Source::WebsiteURL,
        Some(vec![Consent::NameConsent, Consent::ContentConsent])
    );

    let mut customer2 = cs::Customer::new(
        String::from("Jane"), 
        String::from("url2")
    );    

    let mut testimonial2 = Testimonial::new(
        customer2.name.clone(),
        String::from("Didn't like it that much."),
        String::from("2024-02-15"),
        Source::X,
        None,

    );

    println!("*****ORIGINAL TESTIMONIALS*****");
    testimonial1.display_consents();
    println!("\n");
    testimonial2.display_consents();
    println!("\n");

    //Jane adds name consent
    customer2.add_consent(&mut testimonial2, Consent::NameConsent);
    //John removes name consent
    customer1.remove_consent(&mut testimonial1, Consent::NameConsent);

    println!("*****TESTIMONIALS AFTER CUSTOMER UPDATES*****");
    testimonial1.display_consents();
    println!("\n");
    testimonial2.display_consents();

}