use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, &'buf str>,
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&'buf str> {
        self.data.get(key).copied()
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for pair in s.split('&') {
            if pair.is_empty() {
                continue;
            }

            let (key, value) = match pair.find('=') {
                Some(i) => (&pair[..i], &pair[i + 1..]),
                None => (pair, ""),
            };

            data.insert(key, value);
        }

        QueryString { data }
    }
}
