# yukino-dev
![](https://github.com/dark-flames/yukino-dev/actions/workflows/main.yml/badge.svg)
[![](https://tokei.rs/b1/github/dark-flames/yukino-dev)](https://github.com/XAMPPRocky/tokei)

A type-driven and high performance ORM framework

### Features

* Association
* Calculation push-down
* Type driven

### Example

```rust
// entity.rs
#[derive(Entity)]
pub struct Meeting {
    #[id]
    pub id: u32,
    pub start_time: u64,
    pub end_time: u64
}

// main.rs 
fn meeting_length(id: u32) -> QueryResult<u64> {
    Meeting::all()
        .filter(|m| eq!(m.id, id))
        .first()
        .map(|m| m.end_time - m.start_time)
        .exec()
}
```

Generated query:

```SQL
SELECT (m.end_time - m.start_time)
FROM meeting m
WHERE m.id = ? LIMIT 1;
```