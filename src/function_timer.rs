use std::time::Instant;

pub fn measure_time_expr<F:FnOnce()->T, T>(exec:F, label:&str) -> T{
    let start_time = Instant::now();
    let t = exec();
    println!("Time elapsed for expression \"{label}\": {} us", start_time.elapsed().as_micros());
    return t;
}

pub fn measure_time_expr_n_times<F:FnMut()->T, T>(mut exec:F, n:u32, label:&str) -> Vec<T>{
    let mut counter = 0;
    let start_time = Instant::now();
    let mut results : Vec<T> = Vec::new();
    while counter<n {
        results.push(exec());
        counter += 1;
    }
    let micro_seconds = start_time.elapsed().as_micros();
    println!("Time elapsed for {n} iterations of expression \"{label}\": {} us", micro_seconds);
    println!("Average time for each iteration of expression \"{label}\": {} us", micro_seconds as f64 / n as f64);
    return results;
}
