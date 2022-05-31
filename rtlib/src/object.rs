use crate::shapes::Shape;
use crate::util::uid;

pub struct Object<T>
where
    T: Shape,
{
    uid: usize,
    shape: T,
}

impl<T> Object<T>
where
    T: Shape,
{
    pub fn new(shape: T) -> Self {
        Self {
            uid: uid::fetch_uid(),
            shape,
        }
    }
}
