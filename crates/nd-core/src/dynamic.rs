//! Generators behind `${!name}`. Each use produces a fresh value.

use chrono::{Duration, SecondsFormat, Utc};
use fake::faker::internet::en::{SafeEmail, Username};
use fake::faker::lorem::en::{Paragraph, Words};
use fake::faker::name::en::FirstName;
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use rand::RngExt;
use uuid::Uuid;

use crate::error::{Error, Result};

/// Every name usable as `${!name}`.
pub const NAMES: &[&str] = &[
    "uuidv4",
    "nanoid",
    "now",
    "yesterday",
    "tomorrow",
    "random_date_past",
    "random_date_future",
    "random_int",
    "random_bool",
    "color",
    "random_name",
    "random_username",
    "random_email",
    "random_phone",
    "random_words",
    "random_paragraph",
];

pub(crate) fn invoke(name: &str) -> Result<String> {
    let mut rng = rand::rng();
    return match name {
        "uuidv4" => Ok(Uuid::new_v4().to_string()),
        "nanoid" => Ok(nanoid::nanoid!()),
        "now" => Ok(timestamp(Duration::zero())),
        "yesterday" => Ok(timestamp(Duration::hours(-24))),
        "tomorrow" => Ok(timestamp(Duration::hours(24))),
        "random_date_past" => Ok(timestamp(Duration::days(rng.random_range(-3650..0)))),
        "random_date_future" => Ok(timestamp(Duration::days(rng.random_range(1..3650)))),
        "random_int" => Ok(rng.random::<i64>().to_string()),
        "random_bool" => Ok(rng.random_bool(0.5).to_string()),
        "color" => Ok(format!("#{:06x}", rng.random_range(0..=0xffffffu32))),
        "random_name" => Ok(FirstName().fake()),
        "random_username" => Ok(Username().fake()),
        "random_email" => Ok(SafeEmail().fake()),
        "random_phone" => Ok(PhoneNumber().fake()),
        "random_words" => Ok(Words(3..10).fake::<Vec<String>>().join(" ")),
        "random_paragraph" => Ok(Paragraph(1..4).fake()),
        _ => Err(Error::UnknownDynamic(name.to_string())),
    };
}

fn timestamp(offset: Duration) -> String {
    (Utc::now() + offset).to_rfc3339_opts(SecondsFormat::Secs, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_generator_produces_a_value() {
        for name in NAMES {
            assert!(!invoke(name).unwrap().is_empty(), "{name}");
        }
    }
}
