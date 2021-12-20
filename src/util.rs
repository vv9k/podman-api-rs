pub mod url {
    use std::{borrow::Borrow, string::ToString};
    use url::form_urlencoded;

    pub fn construct_ep<E, Q>(ep: E, query: Option<Q>) -> String
    where
        E: Into<String>,
        Q: AsRef<str>,
    {
        let mut ep = ep.into();
        if let Some(query) = query {
            append_query(&mut ep, query);
        }
        ep
    }

    pub fn append_query<Q>(ep: &mut String, query: Q)
    where
        Q: AsRef<str>,
    {
        ep.push('?');
        ep.push_str(query.as_ref());
    }

    pub fn encoded_pair<K, V>(key: K, val: V) -> String
    where
        K: AsRef<str> + 'static,
        V: ToString,
    {
        form_urlencoded::Serializer::new(String::new())
            .append_pair(key.as_ref(), &val.to_string())
            .finish()
    }

    pub fn encoded_pairs<I, K, V>(iter: I) -> String
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        iter.into_iter()
            .fold(
                form_urlencoded::Serializer::new(String::new()),
                |mut acc, v| {
                    let &(ref k, ref v) = v.borrow();
                    let k = k.as_ref();
                    let v = v.as_ref();
                    if v.is_empty() {
                        acc.append_key_only(k);
                    } else {
                        acc.append_pair(k, v);
                    }
                    acc
                },
            )
            .finish()
    }
}

#[cfg(feature = "chrono")]
pub mod datetime {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;

    pub(crate) fn datetime_from_unix_timestamp<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let timestamp = chrono::NaiveDateTime::from_timestamp(i64::deserialize(deserializer)?, 0);
        Ok(DateTime::<Utc>::from_utc(timestamp, Utc))
    }

    pub(crate) fn datetime_from_nano_timestamp<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let timestamp_nano = u64::deserialize(deserializer)?;
        let timestamp = chrono::NaiveDateTime::from_timestamp(
            (timestamp_nano / 1_000_000_000) as i64,
            (timestamp_nano % 1_000_000_000) as u32,
        );
        Ok(DateTime::<Utc>::from_utc(timestamp, Utc))
    }
}
