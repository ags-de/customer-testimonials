use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Source {
    WebsiteURL,
    Facebook,
    Instagram,
    X,
}

#[derive(Debug)]
pub enum TestimonialStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Consent {
    NameConsent,
    ContentConsent,
    LogoConsent,
}

#[derive(Debug)]
pub struct ConsentStatus {
    pub source_status: HashMap<Option<Consent>, bool>,
}

pub struct Testimonial {
    name: String,
    content: String,
    date: String,
    source: Source,
    initial_consent: Option<Vec<Consent>>,
    consent_status: ConsentStatus,
}

impl Testimonial {
    pub fn new(name: String, content: String, date: String, source: Source, consents: Option<Vec<Consent>>) -> Testimonial {
        let mut consent_status = HashMap::new();
        if let Some(consents) = &consents {
            for consent in consents {
                consent_status.insert(Some(consent.clone()), false);
            }
        }

        Testimonial {
            name,   
            content,
            date,
            source,
            initial_consent: Option<Vec<Consent>>,
            status: ConsentStatus {
                consent_status,
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

    pub fn update_consent(&mut self) {
        if let Some(sources) = &self.initial_sources {
            for source in sources {
                if let Some(status) = self.status.source_status.get_mut(&Some(source.clone())) {
                    *status = TestimonialStatus::Active;
                }
            }
        }
    }

    pub fn display_consent(&self) {
        println!("Testimonial: {}", self.name);
        for source in &[Source::WebsiteURL, Source::Facebook, Source::Instagram, Source::X] {
            let status = self.status.source_status.get(&Some(source.clone())).unwrap_or(&TestimonialStatus::Inactive);
            println!("{:?}: {:?}", source, status);
        }
    }
}