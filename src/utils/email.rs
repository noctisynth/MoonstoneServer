use lazy_static::lazy_static;
use regex::Regex;
pub struct EmailValidatorCacher {
    regex: &'static Regex,
}

impl EmailValidatorCacher {
    pub fn new() -> EmailValidatorCacher {
        lazy_static! {
            static ref EMAIL_REGEX: Regex =
                Regex::new(r"^[a-zA-Z0-9._%+-]+@(tutanota.com|tuta.com)$")
                    .expect("Invalid regex pattern");
        }
        EmailValidatorCacher {
            regex: &EMAIL_REGEX,
        }
    }

    pub fn validate(&self, email: &str) -> bool {
        self.regex.is_match(email)
    }
}

lazy_static! {
    pub static ref EMAIL_VALIDATOR: EmailValidatorCacher = EmailValidatorCacher::new();
}

#[cfg(test)]
mod tests {
    use super::EMAIL_VALIDATOR;

    #[test]
    fn test_email_validator() {
        let email1 = "test@tutanota.com";
        let email2 = "test@tuta.com";
        let email3 = "test@test.com";

        assert!(EMAIL_VALIDATOR.validate(email1));
        assert!(EMAIL_VALIDATOR.validate(email2));
        assert!(!EMAIL_VALIDATOR.validate(email3));
    }
}
