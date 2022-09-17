use serde::ser::SerializeMap;
use std::slice;
use serde::Serialize;
use std::collections::HashMap;
use ijson::IValue;

type Pair = (String, IValue);

#[derive(Debug, PartialEq, Eq)]
pub struct Params(Vec<Pair>);

impl Params {
    pub fn new() -> Params {
        Params(Vec::new())
    }

    pub fn push<K: ToString>(&mut self, key: K, value: IValue) {
        self.0.push((key.to_string(), value))
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

//https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210
pub struct PairsIter<K, V, I>(I)
where
    I: ExactSizeIterator<Item = (K, V)>,
    K: ToString,
    V: Serialize;


impl<K, V, I> TryFrom<PairsIter<K, V, I>> for Params
where
    I: ExactSizeIterator<Item = (K, V)>,
    K: ToString,
    V: Serialize
{
    type Error = serde_json::Error;

    fn try_from(pairs_iter: PairsIter<K, V, I>) -> Result<Self, Self::Error>{
        let mut vector = Vec::with_capacity(pairs_iter.0.len());

        for (k, v) in pairs_iter.0 {
            vector.push((k.to_string(), ijson::to_value(v)?));
        }

        Ok(Params(vector))
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

impl<K, V, const N: usize> TryFrom<[(K, V); N]> for Params
where
    K: ToString,
    V: Serialize
{
    type Error = serde_json::Error;
    
    fn try_from(array: [(K, V); N]) -> Result<Self, Self::Error> {
        let mut vector = Vec::with_capacity(N);

        for (key, value) in array {
            vector.push((key.to_string(), ijson::to_value(value)?))
        }

        Ok(Params(vector))
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
    use ijson::ijson;

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

    #[test]
    fn try_from_array() {
        let array = [("user_id", 1)];

        let params = Params::try_from(array).unwrap();

        assert_eq!(params, Params(vec![
            ("user_id".to_string(), ijson!(1))
        ]));
    }

    #[test]
    fn try_from_pairs_iter() {
        let array = vec![("user_id", 1)];

        let params = Params::try_from(PairsIter(array.into_iter())).unwrap();

        assert_eq!(params, Params(vec![
            ("user_id".to_string(), ijson!(1))
        ]));
    }

    #[test]
    fn from_string_pairs_iter() {
        let array = vec![("user_ids", "1,2,3,4,5")];

        let params = Params::try_from(PairsIter(array.into_iter())).unwrap();

        assert_eq!(params, Params(vec![
            ("user_ids".to_string(), "1,2,3,4,5".into())
        ]));
    }
}