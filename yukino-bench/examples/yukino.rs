use yukino_bench::*;
use yukino_bench::yukino_benches::YukinoHandler;

const URL: &str = "mysql://root@localhost:3306/bench";

fn main() {
    let mut handler = YukinoHandler::create(URL);
    for _ in 0..1 {
        handler.bench_associated_calc();
    }
}
