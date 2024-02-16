mod testimonial;
use testimonial as ts;

fn main() {
    let mut testimonial1 = ts::Testimonial::new(
        String::from("John"),
        String::from("Great product!"),
        String::from("2024-02-16"),
        Some(vec![ts::Source::WebsiteURL, ts::Source::Facebook]),
    );

    testimonial1.update_status();
    testimonial1.display_status();

    let mut testimonial2 = ts::Testimonial::new(
        String::from("Jane"),
        String::from("Didn't like it that much."),
        String::from("2024-02-15"),
        None, // No sources
    );

    testimonial2.update_status();
    testimonial2.display_status();
}