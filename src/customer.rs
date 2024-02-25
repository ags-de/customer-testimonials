use crate::testimonial::{Consent, ConsentStatus, Testimonial};

pub struct Customer {
    pub name: String,
    logo: String, //url of a logo
}

impl Customer {
    pub fn new(name: String, logo: String) -> Customer {
        Customer {
            name,
            logo,
        }
    }

    pub fn add_consent(&mut self, testimonial: &mut Testimonial, consent: Consent) {
        if let Some(consent_status) = testimonial.consent_status.consentst.get_mut(&Some(consent.clone())) {
            *consent_status = true;
        } else {
            testimonial.consent_status.consentst.insert(Some(consent), true);
        }
    }

    pub fn remove_consent(&mut self, testimonial: &mut Testimonial, consent: Consent) {
        if let Some(consent_status) = testimonial.consent_status.consentst.get_mut(&Some(consent.clone())) {
            *consent_status = false;
        } else {
            testimonial.consent_status.consentst.insert(Some(consent), false);
        }
    }
}
