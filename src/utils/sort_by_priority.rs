pub trait SortByPriority<'a, U> {
    /// Sorts the items in the collection by the priority function provided.
    fn sort_by_priority<F>(&mut self, priority_func: F)
    where
        F: Fn(&U) -> u32;
}

impl<'a, T, U> SortByPriority<'a, U> for T
where
    T: AsMut<[U]>,
{
    /// Sorts the items in the collection by the priority function provided.
    fn sort_by_priority<F>(&mut self, priority_func: F)
    where
        F: Fn(&U) -> u32,
    {
        self.as_mut()
            .sort_by(|a: &U, b: &U| priority_func(a).cmp(&priority_func(b)))
    }
}
