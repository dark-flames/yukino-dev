# yukino-dev
![](https://github.com/dark-flames/yukino-dev/actions/workflows/main.yml/badge.svg)
[![](https://tokei.rs/b1/github/dark-flames/yukino-dev)](https://github.com/XAMPPRocky/tokei)

A type-safe, high performance ORM framework

### Features

* Association
* Calculation push-down
* Type safe

### Example

```rust
// entity.rs
#[Entity]
pub struct Meeting {
    #[ID]
    pub id: u32,
    pub start_time: u64,
    pub end_time: u64
}

// main.rs 
fn meeting_length(id: u32) -> QueryResult<u64> {
    Meeting::all()
        .filter(|m| cmp!(m.id == id))
        .first()
        .map(|m, _| m.end_time - m.start_time)
        .exec()
}
```

Generated query:

```SQL
SELECT (m.end_time - m.start_time)
FROM meeting m
WHERE m.id = ? LIMIT 1;
```