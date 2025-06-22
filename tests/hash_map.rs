use entry_put_ext::prelude::*;
use std::collections::HashMap;

#[test]
fn put() {
    with_new();
    with_overwrite();

    fn with_new() {
        let mut map = HashMap::new();
        let target = map.entry("X");
        let result = *target.put(true);
        assert_eq!(result, map["X"]);
    }

    fn with_overwrite() {
        let mut map = HashMap::from([("X", false)]);
        let target = map.entry("X");
        let result = *target.put(true);
        assert_eq!(result, true);
        assert_eq!(result, map["X"]);
    }
}
