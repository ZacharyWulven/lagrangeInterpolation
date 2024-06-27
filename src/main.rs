use  std::{ env, process, error::Error };
use lagrangeInterpolation::Config;

fn main() {

    let args = env::args();
    eprintln!("{:?}", args);

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lagrangeInterpolation::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }


}


