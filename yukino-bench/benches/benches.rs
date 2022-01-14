use criterion::*;
use sqlx::{Connection, MySqlConnection};

use yukino_bench::*;
use yukino_bench::diesel_benches::DieselHandler;
use yukino_bench::sqlx_benches::SqlxHandler;
use yukino_bench::yukino_benches::YukinoHandler;

#[allow(dead_code)]
const INSERT: &[usize] = &[1, 16, 64, 256, 1024, 4096];
#[allow(dead_code)]
const FETCH_ALL: &[usize] = &[1, 16, 64, 256, 1024, 4096];
#[allow(dead_code)]
const ASSOC_CALC: &[usize] = &[1, 16, 64, 128, 256];
#[allow(dead_code)]
const ASSOC_ZIP: &[usize] = &[1, 16, 64, 128, 256];

const URL: &str = "mysql://root@localhost:3306/bench";

#[allow(dead_code)]
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

        sqlx::query("TRUNCATE TABLE examination;")
            .execute(&mut conn)
            .await
            .unwrap();

        sqlx::query("SET FOREIGN_KEY_CHECKS = 1;")
            .execute(&mut conn)
            .await
            .unwrap();
    })
}

#[allow(dead_code)]
fn bench_insert(c: &mut Criterion) {
    for (name, introduction_size) in
        vec![("insert_small_objects", 10), ("insert_big_objects", 2000)]
    {
        let mut group = c.benchmark_group(name);
        for size in INSERT {
            let data = generate_user(*size, introduction_size);
            drop_all();
            group.bench_with_input(
                BenchmarkId::new(YukinoHandler::orm_name(), size),
                &size,
                |c, _| {
                    let mut handler = YukinoHandler::create(URL);
                    let data = YukinoHandler::convert_users(data.clone());
                    c.iter(|| handler.bench_insert(data.clone()))
                },
            );
            drop_all();
            group.bench_with_input(
                BenchmarkId::new(DieselHandler::orm_name(), size),
                &size,
                |c, _| {
                    let mut handler = DieselHandler::create(URL);
                    let data = DieselHandler::convert_users(data.clone());
                    c.iter(|| handler.bench_insert(data.clone()))
                },
            );
            drop_all();
            group.bench_with_input(
                BenchmarkId::new(SqlxHandler::orm_name(), size),
                &size,
                |c, _| {
                    let mut handler = SqlxHandler::create(URL);
                    let data = SqlxHandler::convert_users(data.clone());
                    c.iter(|| handler.bench_insert(data.clone()))
                },
            );
        }
        group.finish();
    }
}

#[allow(dead_code)]
fn bench_fetch_all(c: &mut Criterion) {
    for (name, introduction_size) in vec![("fetch_small_objects", 10), ("fetch_big_objects", 2000)]
    {
        let mut group = c.benchmark_group(name);
        for size in FETCH_ALL {
            drop_all();
            let mut handler = YukinoHandler::create(URL);

            handler.bench_insert(YukinoHandler::convert_users(generate_user(
                *size,
                introduction_size,
            )));

            group.bench_with_input(
                BenchmarkId::new(YukinoHandler::orm_name(), size),
                size,
                |c, _| {
                    let mut handler = YukinoHandler::create(URL);
                    c.iter(|| handler.bench_fetch_all())
                },
            );

            group.bench_with_input(
                BenchmarkId::new(DieselHandler::orm_name(), size),
                size,
                |c, _| {
                    let mut handler = DieselHandler::create(URL);
                    c.iter(|| handler.bench_fetch_all())
                },
            );

            group.bench_with_input(
                BenchmarkId::new(SqlxHandler::orm_name(), size),
                size,
                |c, _| {
                    let mut handler = SqlxHandler::create(URL);
                    c.iter(|| handler.bench_fetch_all())
                },
            );
        }
        group.finish();
    }
}

#[allow(dead_code)]
fn bench_zip_association(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_zip_association");
    for size in ASSOC_ZIP {
        drop_all();
        generate_data(URL, *size, 1000, 1000);

        group.bench_with_input(
            BenchmarkId::new(YukinoHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = YukinoHandler::create(URL);
                c.iter(|| handler.bench_zip_association())
            },
        );

        group.bench_with_input(
            BenchmarkId::new(DieselHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = DieselHandler::create(URL);
                c.iter(|| handler.bench_zip_association())
            },
        );

        group.bench_with_input(
            BenchmarkId::new(SqlxHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = SqlxHandler::create(URL);
                c.iter(|| handler.bench_zip_association())
            },
        );
    }
    group.finish();
}

#[allow(dead_code)]
fn bench_association_calc(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_association_calculation");
    for size in ASSOC_CALC {
        drop_all();
        generate_data(URL, *size, 100, 100);

        group.bench_with_input(
            BenchmarkId::new(YukinoHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = YukinoHandler::create(URL);
                c.iter(|| handler.bench_associated_calc())
            },
        );

        group.bench_with_input(
            BenchmarkId::new(DieselHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = DieselHandler::create(URL);
                c.iter(|| handler.bench_associated_calc())
            },
        );

        group.bench_with_input(
            BenchmarkId::new(SqlxHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = SqlxHandler::create(URL);
                c.iter(|| handler.bench_associated_calc())
            },
        );
    }
    group.finish();
}

criterion::criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_insert,
        bench_fetch_all,
        bench_zip_association,
        bench_association_calc

);

criterion::criterion_main!(benches);
