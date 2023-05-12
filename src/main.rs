mod params;
mod threads_runner;

use crate::params::Params;
use crate::threads_runner::threads_runner;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let params = match Params::new(&args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(0);
        },
    };
    let ip = params.ip();
    let n_threads = params.n_threads();
    let iterations_per_thread = u16::MAX.checked_div(*n_threads).unwrap_or(1);

    println!("Number of open ports for host: {ip}");
    threads_runner(iterations_per_thread, *ip, *n_threads);
}
