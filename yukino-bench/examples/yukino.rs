use std::time::Instant;

use yukino_bench::*;
use yukino_bench::yukino_benches::YukinoHandler;

const URL: &str = "mysql://root@localhost:3306/bench";

fn main() {
    let start = Instant::now();
    let mut handler = YukinoHandler::create(URL);
    for _ in 0..1000 {
        handler.bench_fetch_all();
    }

    println!("cost: {}", start.elapsed().as_millis())
}
