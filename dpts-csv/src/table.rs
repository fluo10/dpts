use dpts_error::Error;
use super::CsvRecord;
pub struct CsvTable{
    inner: Vec<CsvRecord>
}

impl CsvTable {
    pub fn into_inner(self) -> Vec<CsvRecord> {
        self.inner
    }   
}

impl From<Vec<CsvRecord>> for CsvTable {
    fn from(v: Vec<CsvRecord>) -> Self {
        Self {
            inner: v,
        }
    }
}