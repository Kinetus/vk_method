use super::Params;
use serde::Serialize;

/// Newtype for conversion array of pairs into [Params]
///
/// See also <https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210>
/// 
/// # Example
/// ```
/// use vk_method::{Params, PairsArray};
/// use ijson::ijson;
/// 
/// let array = [("user_id", 1)];
/// 
/// let params = Params::try_from(PairsArray(array)).unwrap();
/// 
/// assert_eq!(
///     params,
///     Params(vec![("user_id".to_string(), ijson!(1))])
/// );
///```
pub struct PairsArray<K, V, const N: usize>(pub [(K, V); N])
where
    K: ToString,
    V: Serialize;

impl<K, V, const N: usize> TryFrom<PairsArray<K, V, N>> for Params
where
    K: ToString,
    V: Serialize
{
    type Error = serde_json::Error;
    
    fn try_from(array: PairsArray<K, V, N>) -> Result<Self, Self::Error> {
        let mut vector = Vec::with_capacity(N);

        for (key, value) in array.0 {
            vector.push((key.to_string(), ijson::to_value(value)?))
        }

        Ok(Params(vector))
    }
}