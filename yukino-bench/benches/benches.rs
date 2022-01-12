use criterion::*;
use sqlx::{Connection, MySqlConnection};

use yukino_bench::*;
use yukino_bench::diesel_benches::DieselHandler;
use yukino_bench::sqlx_benches::SqlxHandler;
use yukino_bench::yukino_benches::YukinoHandler;

const INSERT: &[(&str, usize, usize)] = &[
    ("single_small_object", 1, 1),
    ("single_big_object", 1, 10000),
    ("100_small_object", 100, 1),
    ("100_big_object", 100, 10000),
    ("1000_small_object", 1000, 1),
    ("1000_big_object", 1000, 10000),
    ("5000_small_object", 5000, 1),
    ("5000_big_object", 5000, 1000),
];
const FETCH_ALL: &[(&str, usize, usize)] = &[
    ("single_small_object", 1, 1),
    ("single_big_object", 1, 10000),
    ("100_small_object", 100, 1),
    ("100_big_object", 100, 10000),
    ("1000_small_object", 1000, 1),
    ("1000_big_object", 1000, 10000),
    ("5000_small_object", 5000, 1),
    ("5000_big_object", 5000, 1000),
];

const URL: &str = "mysql://root@localhost:3306/bench";

fn drop_all() {
    use tokio::runtime::Runtime;

    Runtime::new().unwrap().block_on(async {
        let mut conn = MySqlConnection::connect(URL).await.unwrap();
        sqlx::query("SET FOREIGN_KEY_CHECKS = 0;")
            .execute(&mut conn)
            .await
            .unwrap();
        sqlx::query("TRUNCATE TABLE user;")
            .execute(&mut conn)
            .await
            .unwrap();

        sqlx::query("SET FOREIGN_KEY_CHECKS = 1;")
            .execute(&mut conn)
            .await
            .unwrap();
    })
}

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_insert");
    for (name, size, introduction_size) in INSERT {
        let data = generate_user(*size, *introduction_size);
        drop_all();
        group.bench_with_input(
            BenchmarkId::new(format!("{}.insert", YukinoHandler::orm_name()), name),
            &data,
            |c, i| {
                let mut handler = YukinoHandler::create(URL);
                let data = YukinoHandler::convert_users(i.clone());
                c.iter(|| handler.bench_insert(data.clone()))
            },
        );
        drop_all();
        group.bench_with_input(
            BenchmarkId::new(format!("{}.insert", DieselHandler::orm_name()), name),
            &data,
            |c, i| {
                let mut handler = DieselHandler::create(URL);
                let data = DieselHandler::convert_users(i.clone());
                c.iter(|| handler.bench_insert(data.clone()))
            },
        );
        drop_all();
        group.bench_with_input(
            BenchmarkId::new(format!("{}.insert", SqlxHandler::orm_name()), name),
            &data,
            |c, i| {
                let mut handler = SqlxHandler::create(URL);
                let data = SqlxHandler::convert_users(i.clone());
                c.iter(|| handler.bench_insert(data.clone()))
            },
        );
    }
    group.finish();
}

fn bench_fetch_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_fetch_all");
    for (name, size, introduction_size) in FETCH_ALL {
        drop_all();
        let mut handler = YukinoHandler::create(URL);
        println!("Insert: {} x {}", size, introduction_size);
        handler.bench_insert(YukinoHandler::convert_users(generate_user(
            *size,
            *introduction_size,
        )));

        let id = format!("{}.fetch_all/{}", YukinoHandler::orm_name(), name);
        group.bench_function(id, |c| {
            let mut handler = YukinoHandler::create(URL);
            c.iter(|| handler.bench_fetch_all())
        });

        let id = format!("{}.fetch_all/{}", DieselHandler::orm_name(), name);
        group.bench_function(id, |c| {
            let mut handler = DieselHandler::create(URL);
            c.iter(|| handler.bench_fetch_all())
        });

        let id = format!("{}.fetch_all/{}", SqlxHandler::orm_name(), name);
        group.bench_function(id, |c| {
            let mut handler = DieselHandler::create(URL);
            c.iter(|| handler.bench_fetch_all())
        });
    }
    group.finish();
}

criterion::criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = //bench_insert,
        bench_fetch_all
);

criterion::criterion_main!(benches);
