pub struct CsvWriter<T> {
    inner: T
}

impl<T> CsvWriter<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }   
}