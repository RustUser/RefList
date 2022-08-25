///This trait is used to define the maximum index, minimum index and the current index of a value.
pub trait ToIndex {
    ///This is the maximum index of a value. For example, the MAX_INDEX of a byte would be 255.
    const MAX_INDEX: usize;

    ///This is the minimum index of a value. For example, the MIN_INDEX of a byte would be 0.
    const MIN_INDEX: usize;

    ///This is the function that converts the value into an index number. For example, if self were 4(byte), the index would be 4.
    fn to_index(&self) -> usize;
}