mod params;
pub use params::{Params, PairsIter};

pub struct Method {
    pub name: String,
    pub params: Params
}

impl Method {
    pub fn new<T: ToString>(name: T, params: Params) -> Method {
        Method { name: name.to_string(), params }
    }
}
