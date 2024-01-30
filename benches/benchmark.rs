use advent_of_code::solve;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench {
    ($function:ident) => {
        fn $function(c: &mut Criterion) {
            let mut year_day_part = stringify!($function)
                .strip_prefix("benchmark_")
                .unwrap()
                .split("_");
            let year = year_day_part.next().unwrap().parse().unwrap();
            let day = year_day_part.next().unwrap().parse().unwrap();
            let part = year_day_part.next().unwrap().parse().unwrap();
            let input = std::fs::read_to_string(&format!("input/{}/{:02}.txt", year, day)).unwrap();
            c.bench_function(stringify!($function), |b| {
                b.iter(|| {
                    solve(
                        black_box(year),
                        black_box(day),
                        black_box(part),
                        black_box(input.clone()),
                    )
                })
            });
        }
    };
}

macro_rules! benches {
    ($($function:ident),* $(,)?) => {
        $(bench!($function);)*
        criterion_group!(benches, $($function,)*);
        criterion_main!(benches);
    };
}

benches!(
    // 2022
    benchmark_2022_01_1,
    benchmark_2022_01_2,
    benchmark_2022_02_1,
    benchmark_2022_02_2,
    benchmark_2022_03_1,
    benchmark_2022_03_2,
    benchmark_2022_04_1,
    benchmark_2022_04_2,
    benchmark_2022_05_1,
    benchmark_2022_05_2,
    benchmark_2022_06_1,
    benchmark_2022_06_2,
    benchmark_2022_07_1,
    benchmark_2022_07_2,
    benchmark_2022_08_1,
    benchmark_2022_08_2,
    benchmark_2022_09_1,
    benchmark_2022_09_2,
    benchmark_2022_10_1,
    benchmark_2022_10_2,
    benchmark_2022_11_1,
    benchmark_2022_11_2,
    benchmark_2022_12_1,
    benchmark_2022_12_2,
    // 2023
    benchmark_2023_01_1,
    benchmark_2023_01_2,
    benchmark_2023_02_1,
    benchmark_2023_02_2,
    benchmark_2023_03_1,
    benchmark_2023_03_2,
    benchmark_2023_04_1,
    benchmark_2023_04_2,
    benchmark_2023_05_1,
    benchmark_2023_05_2,
    benchmark_2023_06_1,
    benchmark_2023_06_2,
    benchmark_2023_07_1,
    benchmark_2023_07_2,
    benchmark_2023_08_1,
    benchmark_2023_08_2,
    benchmark_2023_09_1,
    benchmark_2023_09_2,
    benchmark_2023_10_1,
    benchmark_2023_10_2,
    benchmark_2023_11_1,
    benchmark_2023_11_2,
    benchmark_2023_12_1,
    benchmark_2023_12_2,
    benchmark_2023_13_1,
    benchmark_2023_13_2,
    benchmark_2023_14_1,
    benchmark_2023_14_2,
    benchmark_2023_15_1,
    benchmark_2023_15_2,
    benchmark_2023_16_1,
    benchmark_2023_16_2,
    benchmark_2023_17_1,
    benchmark_2023_17_2,
    benchmark_2023_18_1,
    benchmark_2023_18_2,
);
