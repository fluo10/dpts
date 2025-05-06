pub struct CsvReader<T> {
    inner: T,
}

impl<T> CsvReader<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }   
}
