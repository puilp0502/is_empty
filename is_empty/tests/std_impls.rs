#[cfg(feature = "std_impls")]
mod test {
    #![allow(dead_code)]
    use is_empty::IsEmpty;
    use std::collections::*;
    #[test]
    fn btreemap_is_empty() {
        let mut map = BTreeMap::<u32, ()>::new();
        assert!(IsEmpty::is_empty(&map));
        map.insert(1, ());
        assert!(!IsEmpty::is_empty(&map));
    }
    #[test]
    fn btreeset_is_empty() {
        let mut map = BTreeSet::<u32>::new();
        assert!(IsEmpty::is_empty(&map));
        map.insert(1);
        assert!(!IsEmpty::is_empty(&map));
    }

    #[test]
    fn hashmap_is_empty() {
        let mut map = BTreeMap::<u32, ()>::new();
        assert!(IsEmpty::is_empty(&map));
        map.insert(1, ());
        assert!(!IsEmpty::is_empty(&map));
    }
    #[test]
    fn hashset_is_empty() {
        let mut map = HashSet::<u32>::new();
        assert!(IsEmpty::is_empty(&map));
        map.insert(1);
        assert!(!IsEmpty::is_empty(&map));
    }
    #[test]
    fn vec_is_empty() {
        let mut vec = Vec::<u32>::new();
        assert!(IsEmpty::is_empty(&vec));
        vec.push(1);
        assert!(!IsEmpty::is_empty(&vec));
    }
}
