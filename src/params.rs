use std::net::IpAddr;
use std::str::FromStr;

pub struct Params {
  ip: IpAddr,
  n_threads: u16,
}

impl Params {
  const MAX_THREADS: u16 = 1024;
  const HELP_STRING: &'static str = "Usage: port-scanner -j %NUMBER OF THREADS% -i %IP ADDRESS%";

  pub fn new(args: &[String]) -> Result<Self, &str> {
    if args.len() < 5 {
      return Err(Self::HELP_STRING)
    }

    if args[2] == "-h" {
      return Err(Self::HELP_STRING)
    }

    let ip = match IpAddr::from_str(&args[4]) {
      Ok(i) => i,
      Err(_) => return Err("Unable to parse IP address"),
    };

    let n_threads= match args[2].parse::<u16>() {
      Ok(t) => t,
      Err(_) => return Err("Unable to parse number of threads"),
    };

    if n_threads > Self::MAX_THREADS {
      return Err("Threads limit exceeded");
    }

    Ok(Self { ip, n_threads })
  }

  pub fn ip(&self) -> &IpAddr {
    &self.ip
  }

  pub fn n_threads(&self) -> &u16 {
    &self.n_threads
  }
}