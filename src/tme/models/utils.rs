use serde::de;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

#[allow(dead_code)]
pub fn deserialize_value_to_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let v: serde_json::Value = Deserialize::deserialize(d)?;
    Ok(v.to_string())
}

#[allow(dead_code)]
pub fn deserialize_str_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr,
    D: Deserializer<'de>,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            T::from_str(value).map_err(de::Error::custom)
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[allow(dead_code)]
pub fn deserialize_str_or_seq<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr,
    D: Deserializer<'de>,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            println!("AAAAAAAAAAAA  STR");
            T::from_str(value).map_err(de::Error::custom)
        }

        fn visit_seq<S>(self, seq: S) -> Result<T, S::Error>
        where
            S: SeqAccess<'de>,
        {
            println!("AAAAAAAAAAAA  SEQ");
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
