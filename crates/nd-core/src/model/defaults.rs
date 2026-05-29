pub fn default_version() -> String {
    return nd_constants::REQUEST_FILE_DEFAULT_VERSION.into();
}

pub fn default_follow_redirects() -> bool {
    return true;
}

pub fn default_verify_tls() -> bool {
    return true;
}

pub fn default_deprecated() -> bool {
    return false;
}

// serde helper
pub fn is_false(b: &bool) -> bool {
    return !*b;
}
