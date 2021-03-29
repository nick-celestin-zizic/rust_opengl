//TODO replace Vecs with HeapArrays
#[derive(Debug, Clone)]
pub struct HeapArray<T> {
    pub data     : Box<[T]>,
    pub length   : usize,
    pub capacity : usize,
}

#[macro_export]
macro_rules! allocate_heap_array {
    ($type:ty, $size:expr) => {
        {
            let zero = <$type>::default();
            HeapArray {data     : Box::new([zero; $size]),
                       length   : 0,
                       capacity : $size}
        }
    };
}

impl<T> HeapArray<T> {
    pub fn from(vec : Vec<T>) -> Self {
        let len = vec.len();
        let cap = vec.capacity();
        HeapArray { data     : vec.into_boxed_slice(),
                    length   : len,
                    capacity : cap
        }
    }
    pub fn push(&mut self, item : T) {
        debug_assert!(self.length < self.capacity, "overflowed array");
        self.data[self.length] = item;
        self.length += 1;
    }
}


