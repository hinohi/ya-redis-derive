/*!
Yet another Redis derive

## Example

```rust
use redis::{Client, Commands, Connection};
use serde::{Deserialize, Serialize};
use ya_redis_derive::Redis;

// `Redis` depends on `serde::Deserialize` and `Serialize`.
#[derive(Debug, Eq, PartialEq, Redis, Deserialize, Serialize)]
struct MyStruct {
    id: i64,
    name: String,
    description: Option<String>,
    is_genius: bool,
    friend_ids: Vec<i64>,
    some_type: MyEnum,
}

// not necessary derive Redis
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
enum MyEnum {
    A,
    B,
    C,
}

# fn main() {
// You must have redis running before
// example: `docker run --rm -p 6379:6379 redis`
let redis_client = Client::open("redis://localhost").unwrap();
let mut redis_con = redis_client.get_connection().expect("Fail to connect redis server");

let a = MyStruct {
    id: 123,
    name: String::from("名無しの権兵衛"),
    description: Some(String::from("とてもクールなライブラリ")),
    is_genius: true,
    friend_ids: vec![0, 1, 1000000],
    some_type: MyEnum::B,
};
let _: bool = redis_con.set("key-a", &a).expect("Fail to set");
let a2: Option<MyStruct> = redis_con.get("key-a").expect("Fail to get");
assert!(a2.is_some());
assert_eq!(a, a2.unwrap());
# }
```
 */
pub use ya_binary_format::{from_bytes, to_bytes};
pub use ya_redis_proc_macro::Redis;
