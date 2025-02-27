pub struct IgnorePaths<'a, const SIZE: usize> {
    ignored_paths: &'a [&'a str; SIZE],
}

impl<'a, const SIZE: usize> IgnorePaths<SIZE> {
    pub fn new(ignored_paths: &'a [&'a str; SIZE]) -> Self {
        Self { ignored_paths }
    }

    pub fn is_ignored(&self, path: &'a str) -> bool {
        /* todo */
        self.ignored_paths.contains(&path)
    }
}
