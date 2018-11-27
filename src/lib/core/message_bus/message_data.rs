pub trait MessageData {}

impl<T> MessageData for T  where T: Sized {}

impl std::fmt::Debug for MessageData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Some data here (NOTE: This is a stub!)") // TODO: Implement a better formatter
    }
}

impl std::cmp::PartialEq for MessageData {
    fn eq(&self, other: &(dyn MessageData + 'static)) -> bool {
        self == other
    }
}
