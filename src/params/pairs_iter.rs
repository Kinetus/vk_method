use super::Params;
use serde::Serialize;

/// Newtype for conversion iterator of pairs into [Params]
/// 
/// See also <https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210>
/// 
/// # Example
/// ```
/// use vk_method::{Params, PairsIter};
/// use ijson::ijson;
/// 
/// let array = vec![("user_id", 1)];
///
/// let params = Params::try_from(PairsIter(array.into_iter())).unwrap();
///
/// assert_eq!(
///     params,
///     Params(vec![("user_id".to_string(), ijson!(1))])
/// );
///```
pub struct PairsIter<K, V, I>(pub I)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string_pairs_iter() {
        let array = vec![("user_ids", "1,2,3,4,5")];

        let params = Params::try_from(PairsIter(array.into_iter())).unwrap();

        assert_eq!(params, Params(vec![
            ("user_ids".to_string(), "1,2,3,4,5".into())
        ]));
    }
}