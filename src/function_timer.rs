#[macro_export]
macro_rules! benchmark{    
    ($label:expr, $expr:expr) => {
        {
        println!("Starting benchmark: \"{}\"", $label);
        let start_time = std::time::Instant::now();
        let return_value = $expr;
        println!("Benchmark finished - time taken for \"{}\": {} us", $label, start_time.elapsed().as_micros());  
        return_value
        } 
    };
}
