use std::default::Default;



const ITEMS_PER_BUCKET: usize = 2;//16;
const INITIAL_BUCKETS:  usize = 8;//8;

#[derive(Debug)]
struct Bucket<T> {
    data:         [T;    ITEMS_PER_BUCKET],
    occupied:     [bool; ITEMS_PER_BUCKET],
    bucket_index: usize,
    full:         bool,
    count:        usize,
}
impl<T: Default + Copy> Bucket<T> {
    pub fn new(index: usize) -> Self {
        Bucket {
            data:         [T::default(); ITEMS_PER_BUCKET],
            occupied:     [false;        ITEMS_PER_BUCKET],
            bucket_index: index,
            full:         false,
            count:        0
        }
    }
}


#[derive(Debug)]
pub struct BucketArray<T> {
    count:   usize,
    buckets: Vec<Bucket<T>>,
}
impl<T: Default + Copy> BucketArray<T> {
    pub fn new() -> Self {
        BucketArray {
            count: 0,
            buckets: Vec::new(),
        }
    }

}
