use std::fmt::{Debug, Formatter};
use std::mem::size_of_val;
use crate::to_index::ToIndex;

///A RefList is a data structure that is used to wrap a Vec data structure. This data structure is not a silver bullet, however is very efficient in terms of determining if this list contains a value.
///This data structure will most certainly use more storage, and may take more time to initialize. However, this trade off may be worth it.
pub struct RefList<T: ToIndex + Clone + Debug> {
    data: Vec<Option<T>>,
}

impl<T: ToIndex + Clone + Debug> Debug for RefList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut fmt = vec![];
        for v in &self.data {
            if let Some(v) = v {
                fmt.push(v.clone());
            }
        }
        f.write_fmt(format_args!("{:?}", fmt))
    }
}

impl<T: ToIndex + Clone + Debug> RefList<T> {
    pub fn new() -> Self {
        let mut output = Self {
            data: vec![]
        };
        output.clear();
        output
    }
    pub fn push(&mut self, index: T) {
        let i = index.to_index() + T::MIN_INDEX;
        self.data[i] = Some(index);
    }
    pub fn contains(&self, index: &T) -> bool {
        self.data[index.to_index() + T::MIN_INDEX].is_some()
    }
    pub fn remove(&mut self, index: T) {
        self.data[index.to_index() + T::MIN_INDEX] = None;
    }
    pub fn clear(&mut self) {
        self.data = vec![None; self.calculated_size()];
    }
    pub fn len(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool {
        for v in &self.data {
            if v.is_some() {
                return false;
            }
        }
        true
    }
    pub fn size(&self) -> usize { size_of_val(self) }
    fn calculated_size(&self) -> usize {
        T::MIN_INDEX + T::MAX_INDEX
    }
}