use std::hint::black_box;
use std::time::{Duration, Instant};

#[allow(dead_code)]
pub fn benchmark<Func, Factory, FactoryResult, FuncResult>(name: &str, mut f: Func, mut factory: Factory, iterations: usize)
where
    Func: FnMut(FactoryResult) -> FuncResult,
    Factory: FnMut() -> FactoryResult,
{
    let mut total = Duration::ZERO;

    for _ in 0..iterations {
        let input = factory();

        let start = Instant::now();

        let result = black_box(f(black_box(input)));

        total += start.elapsed();
        black_box(result);
    }

    let avg = total / iterations as u32;

    println!("{name}: avg {:?}", avg);
}
