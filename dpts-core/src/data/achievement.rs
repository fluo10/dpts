use super::id::*;

pub struct Achievement<T, U>
where T: MayId,
U: MayId, 
{
    id: T,
    lavel: U,
    count: i8,
}