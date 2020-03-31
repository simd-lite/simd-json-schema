use super::scope;
use chrono;
use publicsuffix::List;
use std::net;
use value_trait::*;

use super::error;

#[allow(missing_copy_implementations)]
pub struct DateTime;

impl<V> super::Validator<V> for DateTime
where
    V: Value + std::clone::Clone + std::convert::From<simd_json::value::owned::Value>,
    <V as Value>::Key: std::borrow::Borrow<str>
        + std::hash::Hash
        + Eq
        + std::convert::AsRef<str>
        + std::fmt::Debug
        + std::string::ToString
        + std::marker::Sync
        + std::marker::Send,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match chrono::DateTime::parse_from_rfc3339(string) {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(error::Format {
                path: path.to_string(),
                detail: "Malformed date time".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Email;

impl<V> super::Validator<V> for Email
where
    V: Value,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match List::empty().parse_email(string) {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(error::Format {
                path: path.to_string(),
                detail: "Malformed email address".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Hostname;

impl<V> super::Validator<V> for Hostname
where
    V: Value,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match List::empty().parse_domain(string) {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(error::Format {
                path: path.to_string(),
                detail: "Malformed hostname".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Ipv4;

impl<V> super::Validator<V> for Ipv4
where
    V: Value,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<net::Ipv4Addr>() {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(error::Format {
                path: path.to_string(),
                detail: "Malformed IP address".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Ipv6;

impl<V> super::Validator<V> for Ipv6
where
    V: Value,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<net::Ipv6Addr>() {
            Ok(_) => super::ValidationState::new(),
            Err(_) => val_error!(error::Format {
                path: path.to_string(),
                detail: "Malformed IP address".to_string()
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Uuid;

impl<V> super::Validator<V> for Uuid
where
    V: Value,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<uuid::Uuid>() {
            Ok(_) => super::ValidationState::new(),
            Err(err) => val_error!(error::Format {
                path: path.to_string(),
                detail: format!("Malformed UUID: {:?}", err)
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Uri;

impl<V> super::Validator<V> for Uri
where
    V: Value,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match url::Url::parse(string) {
            Ok(_) => super::ValidationState::new(),
            Err(err) => val_error!(error::Format {
                path: path.to_string(),
                detail: format!("Malformed URI: {}", err)
            }),
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct UriReference;

impl<V> super::Validator<V> for UriReference
where
    V: Value,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        let base_url = url::Url::parse("http://example.com/").unwrap();

        match base_url.join(string) {
            Ok(_) => super::ValidationState::new(),
            Err(err) => val_error!(error::Format {
                path: path.to_string(),
                detail: format!("Malformed URI reference: {}", err)
            }),
        }
    }
}
