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
    #[test]
    fn complex_struct_is_empty() {
        use std::ffi::{OsString};
        use std::collections::{HashSet, HashMap};
        use is_empty::IsEmpty;
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize, IsEmpty)]
        struct Bar {
            owned_string: String,
            os_string: OsString,

            vec: Vec<String>,
            set: HashSet<String>,
            map: HashMap<String, String>,
        }

        let mut bar = Bar {
            owned_string: String::new(),
            os_string: OsString::new(),
            vec: vec![],
            set: HashSet::new(),
            map: HashMap::new(),
        };
        assert!(bar.is_empty());
        bar.owned_string = String::from("hello");
        assert!(!bar.is_empty());
    }
}
