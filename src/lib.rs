/*! Map entry extension for put operations.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

This crate extends result type of the `entry` methods of the maps ([`HashMap`]
and [`BTreeMap`]) by adding an `put` methods. These `put` methods are helper
method to set a value of an entry and get a reference to it. This makes common
code about setting and getting shortened a little.

# Examples

```
use std::collections::HashMap;
use entry_put_ext::hash_map::EntryPutExt;

let mut map = HashMap::from([("X", false)]);
let x = *map.entry("X").put(true);
let y = *map.entry("Y").put(true);

assert_eq!(x, map["X"]);
assert_eq!(y, map["Y"]);
```

[`BTreeMap`]: std::collections::BTreeMap
[`HashMap`]: std::collections::HashMap
*/

pub mod btree_map;
pub mod hash_map;
