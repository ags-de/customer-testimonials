mod testimonial;
mod customer;

use testimonial as ts;

fn main() {
    let mut testimonial1 = ts::Testimonial::new(
        String::from("John"),
        String::from("Great product!"),
        String::from("2024-02-16"),
        ts::Source::WebsiteURL,
        Some(vec![ts::Consent::NameConsent, ts::Consent::ContentConsent])
    );

    testimonial1.update_consent();
    testimonial1.display_consent();

    let mut testimonial2 = ts::Testimonial::new(
        String::from("Jane"),
        String::from("Didn't like it that much."),
        String::from("2024-02-15"),
        ts::Source::X,
        None, // No consent

    );

    testimonial2.update_consent();
    testimonial2.display_consent();
}