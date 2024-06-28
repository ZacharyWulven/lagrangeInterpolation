use  std::{ env, process, error::Error };
use lagrangeInterpolation::Config;

fn main() {

    let args = env::args();
    eprintln!("{:?}", args);

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match lagrangeInterpolation::run(config)  {
        Ok(result) => println!("Result is: {}", result),
        Err(err) => {
            eprintln!("Application error: {}", err);
            process::exit(1);
        }
    }

}


