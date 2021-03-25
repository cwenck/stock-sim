pub trait Predicate<T>: Send + Sync {
    fn test(&self, value: T) -> bool;
}

impl<T, F> Predicate<T> for F
where
    F: Fn(T) -> bool + Send + Sync,
{
    fn test(&self, value: T) -> bool {
        self(value)
    }
}
