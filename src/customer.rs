use std::collections::HashMap;

use crate::testimonial::Consent;
use crate::testimonial::Source;
use crate::testimonial::Status;
use crate::testimonial::Testimonial;
use crate::testimonial::TestimonialStatus;

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

    pub fn create_testimonial(name: String, content: String, date: String, sources: Option<Vec<Source>>, consent: Consent) -> Testimonial {
        let mut source_status = HashMap::new();
        if let Some(sources) = &sources {
            for source in sources {
                source_status.insert(Some(source.clone()), TestimonialStatus::Inactive);
            }
        }

        Testimonial {
            name,
            content,
            date,
            initial_sources: sources,
            status: Status {
                source_status,
            },
            consent,
        }
    }
}
