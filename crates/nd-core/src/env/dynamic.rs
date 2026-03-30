//! Built-ins for `${!name}` template placeholders (see [`crate::template::expand_string`]).
//!
//! Each invocation generates a fresh value using the current time and thread RNG.

use chrono::{Duration, Utc};
use fake::faker::internet::en::{SafeEmail, Username};
use fake::faker::lorem::en::Paragraph;
use fake::faker::lorem::raw::Words;
use fake::faker::name::en::FirstName;
use fake::faker::phone_number::en::PhoneNumber;
use fake::locales::EN;
use fake::Fake;
use rand::Rng;
use uuid::Uuid;

use crate::error::{Error, Result};

/// Resolve a `${!name}` function; `name` is the identifier after `!`.
pub(crate) fn invoke(name: &str) -> Result<String> {
    let mut rng = rand::thread_rng();
    match name {
        "nanoid" => Ok(nanoid::nanoid!()),
        "random_name" => Ok(FirstName().fake()),
        "random_email" => Ok(SafeEmail().fake()),
        "uuidv4" => Ok(Uuid::new_v4().to_string()),
        "random_int" => Ok(rng.gen::<i64>().to_string()),
        "random_phone" => Ok(PhoneNumber().fake()),
        "random_words" => Ok(Words(EN, 3..10).fake::<Vec<String>>().join(" ")),
        "random_paragraph" => Ok(Paragraph(1..4).fake()),
        "lorem_ipsum" => Ok(lorem_ipsum_snippet(&mut rng)),
        "random_iso_date_string" => Ok(random_iso_datetime(&mut rng)),
        "now" => Ok(Utc::now().format("%d-%m-%Y").to_string()),
        "random_username" => Ok(Username().fake()),
        "random_bool" => Ok(rng.gen_bool(0.5).to_string()),
        "random_date_past" => Ok(random_datetime_offset_days(&mut rng, -3650..0)),
        "random_date_future" => Ok(random_datetime_offset_days(&mut rng, 1..3650)),
        "yesterday" => Ok((Utc::now() - Duration::hours(24)).to_rfc3339()),
        "tomorrow" => Ok((Utc::now() + Duration::hours(24)).to_rfc3339()),
        "color" => Ok(format!("#{:06x}", rng.gen_range(0x000000..=0xffffff))),
        _ => Err(Error::UnknownDynamicTemplate(name.to_string())),
    }
}

fn lorem_ipsum_snippet(rng: &mut impl Rng) -> String {
    const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";
    let hi = 220.min(LOREM.len());
    let take = rng.gen_range(120..=hi);
    LOREM[..take].to_string()
}

fn random_iso_datetime(rng: &mut impl Rng) -> String {
    let days = rng.gen_range(-5000..5000);
    let secs = rng.gen_range(0..86400);
    let t = Utc::now() + Duration::days(days) + Duration::seconds(secs);
    t.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

fn random_datetime_offset_days(rng: &mut impl Rng, range: std::ops::Range<i64>) -> String {
    let days = rng.gen_range(range);
    let secs = rng.gen_range(0..86400);
    let t = Utc::now() + Duration::days(days) + Duration::seconds(secs);
    t.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}
