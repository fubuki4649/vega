pub trait SortByPriority<'a, U> {
    fn sort_by_priority<F>(&mut self, priority_func: F)
    where
        F: Fn(&U) -> u32;
}

impl<T, U> SortByPriority<'_, U> for T
where
    T: AsMut<[U]>,
{
    fn sort_by_priority<F>(&mut self, priority_func: F)
    where
        F: Fn(&U) -> u32,
    {
        self.as_mut().sort_by_key(|a| priority_func(a));
    }
}
