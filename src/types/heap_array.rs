use crate::types::HeapArray;

impl<T> HeapArray<T> {
}

//heap_array!(Vertex, 1024 * 1024);

#[macro_export]
macro_rules! heap_array {
    ($type:ty, $size:expr) => {
        {
            let zero = <$type>::default();
            HeapArray {data  : Box::new([zero; $size]),
                       count : 0, }
        }
    };
}
