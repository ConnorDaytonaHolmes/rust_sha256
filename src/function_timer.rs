#[macro_export]
macro_rules! benchmark{    
    ($label:literal, $x:block) => {
        println!("Starting benchmark on \"{}\"", $label);
        let start_time = std::time::Instant::now();
        $x;
        println!("Benchmark finished - time taken for \"{}\": {} us", $label, start_time.elapsed().as_micros());
    };

    ($x:block) => {benchmark!("unnamed expression", $x);};
}

#[macro_export]
macro_rules! benchmark_n_times{
    ($label:literal, $n:ident, $x:block) => {
        println!("Starting benchmark...");
        let start_time = std::time::Instant::now();
        for _ in 0..$n {
            $x;
        }
        let micros = start_time.elapsed().as_micros() as f64;
        println!("Benchmark finished - time taken for {} iterations of \"{}\": {} us", $n, $label, micros);
        println!("Average time for each iteration: {} us", micros / ($n as f64));
    };
    ($label:literal, $x:block) => {benchmark!($x, $label);}
}
