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
    pub consentst: HashMap<Option<Consent>, bool>,
}

pub struct Testimonial {
    name: String,
    content: String,
    date: String,
    source: Source,
    pub consent_status: ConsentStatus,
}

impl Testimonial {
    pub fn new(name: String, content: String, date: String, source: Source, consents: Option<Vec<Consent>>) -> Testimonial {
        let mut testimonial = Testimonial {
            name,   
            content,
            date,
            source,
            consent_status: ConsentStatus {
                consentst: HashMap::new(),
            },
        };
        testimonial.update_consent(&consents);
        testimonial
    }

    pub fn update_consent(&mut self, consents: &Option<Vec<Consent>>) {
        if let Some(consents) = consents {
            for consent in consents {
                if let Some(consent_status) = self.consent_status.consentst.get_mut(&Some(consent.clone())) {
                    *consent_status = true;
                }
            }
        }
    }

    pub fn display_consent(&self) {
        println!("Testimonial: {}", self.name);
        for consent in &[Consent::NameConsent, Consent::ContentConsent, Consent::LogoConsent] {
            let consent_status = self.consent_status.consentst.get(&Some(consent.clone())).unwrap_or(&false);
            println!("{:?}: {:?}", consent, consent_status);
        }
    }
}