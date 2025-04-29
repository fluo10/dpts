mod reader;
mod record;
mod table;
mod writer;

pub use reader::CsvReader;
pub use record::CsvRecord;
pub use table::CsvTable;
pub use writer::CsvWriter;

use crate::error::Error;

use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(test)]
pub mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    const RECORD_CSV:&str = r#"timestamp,comment,tag,count
2025-05-01T12:34:56,"test
test",test,1
"#;

    fn get_records() -> Vec<CsvRecord>{
        vec![CsvRecord {
            timestamp: NaiveDate::from_ymd_opt(2025, 5, 1)
                .unwrap().and_hms_micro_opt(12, 34, 56, 0).unwrap(),
            comment: "test
test".to_owned(),
            tag: "test".to_owned(),
            count: 1,
        }]
    }

    #[test]
    fn deserialize_record() {
        println!("{:?}", RECORD_CSV);
        let mut rdr = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .escape(Some(b'\\'))
            .from_reader(RECORD_CSV.as_bytes());
        let mut raw_record = csv::ByteRecord::new();
        let headers = rdr.byte_headers().unwrap().clone();
        println!("{:?}", &headers);
        let mut records: Vec<CsvRecord> = vec![];
        while rdr.read_byte_record(&mut raw_record).unwrap() {
            println!("{:?}", &raw_record);
            let record: CsvRecord = raw_record.deserialize(Some(&headers)).unwrap();
            records.push(record);
        }
        assert_eq!(records, get_records());
    }

    #[test]
    fn serialize_record() {
        let buf : Vec<u8> = Vec::new();
        let mut wtr = csv::Writer::from_writer(buf);
        for record in get_records().into_iter() {
            wtr.serialize(record).unwrap();
        }
        wtr.flush().unwrap();
        assert_eq!(RECORD_CSV, &String::from_utf8(wtr.into_inner().unwrap()).unwrap())
    }

}