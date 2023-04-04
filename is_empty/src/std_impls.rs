use super::*;

use std::collections::*;

macro_rules! impl_simple {
    ($($t:ty),+) => {

    $(
    impl IsEmpty for $t{
        fn is_empty(&self) -> bool {
            self.is_empty()
        }
    }
    )+

    };
}

impl_simple!(str, String, std::ffi::OsStr);

macro_rules! impl_generic_one{

    ($($t:ty),+) => {

    $(
    impl<K> IsEmpty for $t {
        fn is_empty(&self) -> bool {
            self.is_empty()
        }
    }
    )+
    };
}

impl_generic_one!(BTreeSet<K>, HashSet<K>, LinkedList<K>, VecDeque<K>);

macro_rules! impl_generic_two{

    ($($t:ty),+) => {

    $(
    impl<K, V> IsEmpty for $t {
        fn is_empty(&self) -> bool {
            self.is_empty()
        }
    }
    )+
    };
}

impl_generic_two!(BTreeMap<K,V>, HashMap<K,V>);
