entry_put_ext
===

Map entry extension for put operations.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?
This crate extends result type of the `entry` methods of the maps (`HashMap`
and `BTreeMap`) by adding an `put` methods. These `put` methods are helper
method to set a value of an entry and get a reference to it. This makes
common code about setting and getting shortened a little.

```rust
use std::collections::HashMap;
use entry_put_ext::prelude::*;

let mut map = HashMap::from([("X", false)]);
let x = *map.entry("X").put(true);
let y = *map.entry("Y").put(true);

assert_eq!(x, map["X"]);
assert_eq!(y, map["Y"]);
```

## Other options
Without this crate.

`insert` and `get` (Simple, but little slow and not one-liner).

```rust
use std::collections::HashMap;

let mut map = HashMap::from([("X", false)]);
map.insert("X", true);
map.insert("Y", true);
let x = *map.get("X").unwrap();
let y = *map.get("Y").unwrap();

assert_eq!(x, map["X"]);
assert_eq!(y, map["Y"]);
```

`and_modify` and `or_insert` (required `Copy` trait for value type).

```rust
use std::collections::HashMap;

let mut map = HashMap::from([("X", false)]);
let (xv, yv) = (true, true);
let x = *map.entry("X").and_modify(|x| *x = xv).or_insert(xv);
let y = *map.entry("Y").and_modify(|x| *x = yv).or_insert(yv);

assert_eq!(x, map["X"]);
assert_eq!(y, map["Y"]);
```

## Versions

See [CHANGELOG](CHANGELOG.md).
