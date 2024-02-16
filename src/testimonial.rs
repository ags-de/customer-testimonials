use std::collections::HashMap;

#[derive(Debug)]
pub enum TestimonialStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Source {
    WebsiteURL,
    Facebook,
    Instagram,
    X,
}

#[derive(Debug)]
pub struct Status {
    source_status: HashMap<Option<Source>, TestimonialStatus>,
}

pub struct Customer {
    name: String,
    logo: String,
}

pub struct Testimonial {
    name: String,
    content: String,
    date: String,
    initial_sources: Option<Vec<Source>>,
    status: Status,
}

impl Testimonial {
    pub fn new(name: String, content: String, date: String, sources: Option<Vec<Source>>) -> Testimonial {
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
        }
    }

    pub fn update_status(&mut self) {
        if let Some(sources) = &self.initial_sources {
            for source in sources {
                if let Some(status) = self.status.source_status.get_mut(&Some(source.clone())) {
                    *status = TestimonialStatus::Active;
                }
            }
        }
    }

    pub fn display_status(&self) {
        println!("Testimonial: {}", self.name);
        for source in &[Source::WebsiteURL, Source::Facebook, Source::Instagram, Source::X] {
            let status = self.status.source_status.get(&Some(source.clone())).unwrap_or(&TestimonialStatus::Inactive);
            println!("{:?}: {:?}", source, status);
        }
    }
}