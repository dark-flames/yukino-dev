use criterion::*;
use sqlx::{Connection, MySqlConnection};

use yukino_bench::*;
use yukino_bench::diesel_benches::DieselHandler;
use yukino_bench::sea_orm_benches::SeaOrmHandler;
use yukino_bench::sqlx_benches::SqlxHandler;
use yukino_bench::yukino_benches::YukinoHandler;

#[allow(dead_code)]
const INSERT: &[usize] = &[1, 101, 201, 301, 401, 501];
#[allow(dead_code)]
const FETCH_ALL: &[usize] = &[1, 101, 201, 301, 401, 501];
#[allow(dead_code)]
const ASSOC_CALC: &[usize] = &[1, 20, 40, 60, 80, 100];
#[allow(dead_code)]
const ASSOC_ZIP: &[usize] = &[1, 20, 40, 60, 80, 100];

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

        sqlx::query("FLUSH TABLES;")
            .execute(&mut conn)
            .await
            .unwrap();
    })
}

fn bench_insert(c: &mut Criterion, name: &str, introduction_size: usize) {
    let mut group = c.benchmark_group(name);

    group.plot_config(PlotConfiguration::default());
    for size in INSERT {
        let data = generate_user(*size, introduction_size);

        group.bench_with_input(
            BenchmarkId::new(SqlxHandler::orm_name(), size),
            &size,
            |c, _| {
                let mut handler = SqlxHandler::create(URL);
                let data = SqlxHandler::convert_users(data.clone());
                c.iter(|| handler.bench_insert(data.clone()))
            },
        );
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
            BenchmarkId::new(SeaOrmHandler::orm_name(), size),
            &size,
            |c, _| {
                let mut handler = SeaOrmHandler::create(URL);
                let data = SeaOrmHandler::convert_users(data.clone());
                c.iter(|| handler.bench_insert(data.clone()))
            },
        );
        drop_all();
    }
    group.finish();
}

#[allow(dead_code)]
fn bench_insert_big(c: &mut Criterion) {
    bench_insert(c, "insert_big_objects", 3000)
}

#[allow(dead_code)]
fn bench_insert_small(c: &mut Criterion) {
    bench_insert(c, "insert_small_objects", 10)
}

fn bench_fetch_all(c: &mut Criterion, name: &str, introduction_size: usize) {
    let mut group = c.benchmark_group(name);
    group.plot_config(PlotConfiguration::default());
    group.sampling_mode(SamplingMode::Flat);
    for size in FETCH_ALL {
        drop_all();
        let mut handler = YukinoHandler::create(URL);

        handler.bench_insert(YukinoHandler::convert_users(generate_user(
            *size,
            introduction_size,
        )));

        group.bench_with_input(
            BenchmarkId::new(SqlxHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = SqlxHandler::create(URL);
                c.iter(|| handler.bench_fetch_all())
            },
        );

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
            BenchmarkId::new(SeaOrmHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = SeaOrmHandler::create(URL);
                c.iter(|| handler.bench_fetch_all())
            },
        );
    }
    group.finish();
}

#[allow(dead_code)]
fn bench_fetch_all_small(c: &mut Criterion) {
    bench_fetch_all(c, "fetch_small_objects", 10)
}

#[allow(dead_code)]
fn bench_fetch_all_big(c: &mut Criterion) {
    bench_fetch_all(c, "fetch_big_objects", 3000)
}

fn bench_zip_association(c: &mut Criterion, name: &str, comment_size: usize) {
    let mut group = c.benchmark_group(name);
    group.plot_config(PlotConfiguration::default());
    for size in ASSOC_ZIP {
        drop_all();
        generate_data(URL, *size, 50, comment_size);

        group.bench_with_input(
            BenchmarkId::new(SqlxHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = SqlxHandler::create(URL);
                c.iter(|| handler.bench_zip_association())
            },
        );

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
            BenchmarkId::new(SeaOrmHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = SeaOrmHandler::create(URL);
                c.iter(|| handler.bench_zip_association())
            },
        );
    }
    group.finish();
}

#[allow(dead_code)]
fn bench_zip_association_small(c: &mut Criterion) {
    bench_zip_association(c, "zip_associated_small_object", 10)
}
#[allow(dead_code)]
fn bench_zip_association_big(c: &mut Criterion) {
    bench_zip_association(c, "zip_associated_big_object", 3000)
}

fn bench_association_calc(c: &mut Criterion, name: &str, comment_size: usize) {
    let mut group = c.benchmark_group(name);
    group.plot_config(PlotConfiguration::default());
    for size in ASSOC_CALC {
        drop_all();
        generate_data(URL, *size, 1000, comment_size);

        group.bench_with_input(
            BenchmarkId::new(SqlxHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = SqlxHandler::create(URL);
                c.iter(|| handler.bench_associated_calc())
            },
        );

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
            BenchmarkId::new(SeaOrmHandler::orm_name(), size),
            size,
            |c, _| {
                let mut handler = SeaOrmHandler::create(URL);
                c.iter(|| handler.bench_associated_calc())
            },
        );
    }
    group.finish();
}
#[allow(dead_code)]
fn bench_association_calc_small(c: &mut Criterion) {
    bench_association_calc(c, "associated_small_object_computation", 10)
}
#[allow(dead_code)]
fn bench_association_calc_big(c: &mut Criterion) {
    bench_association_calc(c, "associated_big_object_computation", 3000)
}

criterion::criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_insert_small,
        bench_insert_big,
        bench_fetch_all_small,
        bench_fetch_all_big,
        bench_zip_association_small,
        bench_zip_association_big,
        bench_association_calc_small,
        bench_association_calc_big
);

criterion::criterion_main!(benches);
