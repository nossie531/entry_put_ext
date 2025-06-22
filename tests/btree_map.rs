use entry_put_ext::prelude::*;
use std::collections::BTreeMap;

#[test]
fn put() {
    with_new();
    with_overwrite();

    fn with_new() {
        let mut map = BTreeMap::new();
        let target = map.entry("X");
        let result = *target.put(true);
        assert_eq!(result, map["X"]);
    }

    fn with_overwrite() {
        let mut map = BTreeMap::from([("X", false)]);
        let target = map.entry("X");
        let result = *target.put(true);
        assert_eq!(result, true);
        assert_eq!(result, map["X"]);
    }
}
