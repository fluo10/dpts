use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CsvRecord{
    pub timestamp: NaiveDateTime,
    #[serde(with = "string_to_escape")]
    pub comment: String,
    pub tag: String,
    pub count: i32,
}

pub mod string_to_escape {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(d: D) -> Result<String, D::Error> 
    where D: Deserializer<'de>
    {
        Ok(String::deserialize(d)?
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
        )
    }
    pub fn serialize<S>(s: &str, serializer: S) -> Result<S::Ok, S::Error>
    where  S: Serializer
    {
        serializer.serialize_str(&s

            .replace("\n", "\\n")
            .replace("\t", "\\t")
            .replace("\"", "\\\"")
            .replace("\\", "\\\\")
        )
    }
}



#[cfg(test)]
pub mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    const RECORD_CSV:&str = r#"timestamp,comment,tag,count
2025-05-01T12:34:56,test\ntest,test,1"#;

    fn get_record_struct() -> Vec<CsvRecord>{
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
    fn serialize_string() {

    }

    #[test]
    fn deserialize_string() {

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
        assert_eq!(records, get_record_struct());
    }

    #[test]
    fn serialize_record() {
        todo!()
    }



}