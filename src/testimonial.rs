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
    initial_consents: Option<Vec<Consent>>,
    consent_status: ConsentStatus,
}

impl Testimonial {
    pub fn new(name: String, content: String, date: String, source: Source, consents: Option<Vec<Consent>>) -> Testimonial {
        let mut consentst = HashMap::new();
        if let Some(consents) = &consents {
            for consent in consents {
                consentst.insert(Some(consent.clone()), false);
            }
        }

        Testimonial {
            name,   
            content,
            date,
            source,
            initial_consents: consents,
            consent_status: ConsentStatus {
                consentst,
            },

        }
    }

    pub fn update_consent(&mut self) {
        if let Some(consents) = &self.initial_consents {
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