use super::Codec;
use std::str::FromStr;

/// A codec for strings that relies on [`FromStr`] and [`ToString`] to parse.
///
/// This makes simple key / value easy to use for primitive types. It is also useful for encoding simple data structures without depending on serde.
///
/// ## Example
/// ```
/// # use leptos::*;
/// # use leptos_use::storage::{StorageType, use_local_storage, use_session_storage, use_storage, UseStorageOptions, StringCodec};
/// #
/// # pub fn Demo() -> impl IntoView {
/// let (get, set, remove) = use_local_storage::<i32, StringCodec>("my-key");
/// #    view! { }
/// # }
/// ```
#[derive(Clone, Default, PartialEq)]
pub struct StringCodec;

impl<T: FromStr + ToString> Codec<T> for StringCodec {
    type Error = T::Err;

    fn encode(&self, val: &T) -> Result<String, Self::Error> {
        Ok(val.to_string())
    }

    fn decode(&self, str: String) -> Result<T, Self::Error> {
        T::from_str(&str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_codec() {
        let s = String::from("party time 🎉");
        let codec = StringCodec;
        assert_eq!(codec.encode(&s), Ok(s.clone()));
        assert_eq!(codec.decode(s.clone()), Ok(s));
    }
}
