pub trait SortByPriority<'a, U> {
    fn sort_by_priority<F>(&mut self, priority_func: F)
    where
        F: Fn(&U) -> u32;
}

impl<'a, T, U> SortByPriority<'a, U> for T
where
    T: AsMut<[U]>,
{
    fn sort_by_priority<F>(&mut self, priority_func: F)
    where
        F: Fn(&U) -> u32,
    {
        self.as_mut().sort_by(|a, b| priority_func(a).cmp(&priority_func(b)))
    }
}
