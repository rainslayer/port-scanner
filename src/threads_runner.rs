use std::net::{IpAddr, TcpStream};
use std::thread;

pub fn threads_runner(iterations_per_thread: u16, ip: IpAddr, n_threads: u16) {
  let mut threads = Vec::new();

  for i in 0..n_threads {
    let start = i * iterations_per_thread + i;
    let end = start + iterations_per_thread;

    threads.push(thread::spawn(move || {
      for port in start..end {
        match TcpStream::connect(format!("{ip}:{port}")) {
          Ok(_) => println!("{}", port),
          Err(_) => continue
        }
      }
    }));
  }

  for thread in threads {
    thread.join().unwrap();
  }
}