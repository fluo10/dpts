
#[derive(Debug)]
pub struct IdNumber(usize);
#[derive(Debug)]
pub struct IdString(String);
#[derive(Debug)]
pub struct NoId();

#[derive(Debug, PartialEq)]
pub enum IdValue {
    Number(usize),
    String(String)
}

pub trait MayId {
    fn is_id() -> bool;
    fn get_value(&self) -> Option<IdValue>;
}

impl MayId for IdNumber {
    fn is_id() -> bool {
        true
    }
    fn get_value(&self) -> Option<IdValue> {
        Some(IdValue::Number(self.0))
    }
}
impl MayId for IdString {
    fn is_id() -> bool {
        true
    }
    fn get_value(&self) -> Option<IdValue> {
        Some(IdValue::String(self.0.clone()))
    }
}
impl MayId for NoId {
    fn is_id() -> bool {
        false
    }
    fn get_value(&self) -> Option<IdValue> {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_values() {
        assert_eq!(Some(IdValue::Number(1)), IdNumber(1).get_value());
        assert_eq!(Some(IdValue::String("Test".to_string())), IdString("Test".to_string()).get_value());
        assert_eq!(None, NoId().get_value());
    }
}