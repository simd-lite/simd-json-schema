use std::cmp;
use url;
use value_trait::*;

use super::error;
use super::scope;

#[derive(Debug)]
pub enum ItemsKind {
    Schema(url::Url),
    Array(Vec<url::Url>),
}

#[derive(Debug)]
pub enum AdditionalKind {
    Boolean(bool),
    Schema(url::Url),
}

#[allow(missing_copy_implementations)]
pub struct Items {
    pub items: Option<ItemsKind>,
    pub additional: Option<AdditionalKind>,
}

impl<V> super::Validator<V> for Items
where
    V: Value
        + std::clone::Clone
        + std::convert::From<simd_json::value::owned::Value>
        + std::fmt::Display
        + std::marker::Sync
        + std::marker::Send
        + std::cmp::PartialEq,
    <V as Value>::Key: std::borrow::Borrow<str>
        + std::hash::Hash
        + Eq
        + std::convert::AsRef<str>
        + std::fmt::Debug
        + std::string::ToString
        + std::marker::Sync
        + std::marker::Send,
{
    fn validate(&self, val: &V, path: &str, scope: &scope::Scope<V>) -> super::ValidationState {
        let array = nonstrict_process!(val.as_array(), path);

        let mut state = super::ValidationState::new();

        match self.items {
            Some(ItemsKind::Schema(ref url)) => {
                let schema = scope.resolve(url);
                if schema.is_some() {
                    let schema = schema.unwrap();
                    for (idx, item) in array.iter().enumerate() {
                        let item_path = [path, idx.to_string().as_ref()].join("/");
                        state.append(schema.validate_in(item, item_path.as_ref()));
                    }
                } else {
                    state.missing.push(url.clone());
                }
            }
            Some(ItemsKind::Array(ref urls)) => {
                let min = cmp::min(urls.len(), array.len());

                // Validate against schemas
                for idx in 0..min {
                    let schema = scope.resolve(&urls[idx]);
                    let item = &array.get(idx).unwrap();

                    if schema.is_some() {
                        let item_path = [path, idx.to_string().as_ref()].join("/");
                        state.append(schema.unwrap().validate_in(item, item_path.as_ref()))
                    } else {
                        state.missing.push(urls[idx].clone())
                    }
                }

                // Validate agains additional items
                if array.len() > urls.len() {
                    match self.additional {
                        Some(AdditionalKind::Boolean(allow)) if !allow => {
                            state.errors.push(Box::new(error::Items {
                                path: path.to_string(),
                                detail: "Additional items are not allowed".to_string(),
                            }))
                        }
                        Some(AdditionalKind::Schema(ref url)) => {
                            let schema = scope.resolve(url);
                            if schema.is_some() {
                                let schema = schema.unwrap();
                                for (idx, item) in
                                    array.get(urls.len()..).unwrap().iter().enumerate()
                                {
                                    let item_path = [path, idx.to_string().as_ref()].join("/");
                                    state.append(schema.validate_in(item, item_path.as_ref()))
                                }
                            } else {
                                state.missing.push(url.clone())
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }

        state
    }
}
