use std::ops::Range;

pub trait RangeVec<T> {
    fn to_vec(self) -> Vec<T>;
}

impl <T: IntoIterator + Iterator + std::iter::Step> RangeVec<T> for Range<T> {
    fn to_vec(self) -> Vec<T> {
        let mut output = vec![];
        for v in self {
            output.push(v);
        }
        output
    }
}