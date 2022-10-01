use serde::ser::SerializeMap;
use std::slice;
use serde::Serialize;
use std::collections::HashMap;
use ijson::IValue;

pub mod pairs_iter;
pub use pairs_iter::PairsIter;

pub mod pairs_array;
pub use pairs_array::PairsArray;

type Pair = (String, IValue);

#[derive(Debug, PartialEq, Eq)]
pub struct Params(pub Vec<Pair>);

impl Params {
    pub fn new() -> Params {
        Params(Vec::new())
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: ToString,
        V: Into<IValue>
    {
        self.0.push((key.to_string(), value.into()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for Params {
    type Item = Pair;
    type IntoIter = std::vec::IntoIter<Pair>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Params {
    type Item = &'a Pair;
    type IntoIter = slice::Iter<'a, Pair>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Params {
    type Item = &'a mut Pair;
    type IntoIter = slice::IterMut<'a, Pair>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl From<HashMap<String, String>> for Params {
    fn from(map: HashMap<String, String>) -> Self {
        let mut vector = Vec::with_capacity(map.len());

        for (k, v) in map {
            vector.push((k, v.into()));
        }

        Params(vector)
    }
}

impl<K, const N: usize> From<[(K, IValue); N]> for Params
where
    K: ToString,
{
    fn from(array: [(K, IValue); N]) -> Self {
        let mut vector = Vec::with_capacity(N);

        for (key, value) in array {
            vector.push((key.to_string(), value))
        }

        Params(vector)
    }
}

impl Serialize for Params {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;

        for (k, v) in self.into_iter() {
            map.serialize_entry(k, v)?;
        }
        
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn from_hashmap() {
        let hashmap = HashMap::from([
            ("user_id".to_string(), "1".to_string())
        ]);

        let params = Params::from(hashmap);

        assert_eq!(params, Params(vec![
            ("user_id".to_string(), "1".into())
        ]));
    }
}