pub struct Container<T> {
    pub value: T,
}


impl<T> Container <T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}
