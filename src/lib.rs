
use core::f32;
use  std::{ collections, env, error::Error };

pub struct Config {
    pub primordials: Vec<f32>,
    pub anticipations: Vec<f32>,
    pub value: f32,
}

impl Config {
    
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str>  {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();
        let primordials = match args.next() {
            Some(arg) => { convert_arg_str(&arg).expect("parameter1 parse to [f32] fault") },
            None => return Err("Didn't get a primordials arg"),
        };

        let anticipations = match args.next() {
            Some(arg) => { convert_arg_str(&arg).expect("parameter2 parse to [f32] fault") },
            None => return Err("Didn't get a anticipations arg"),
        };

        let value = match args.next() {
            Some(arg) => arg.parse::<f32>().expect("parameter3 parse to f32 fault"),
            None => return Err("Didn't get a anticipations value"),
        };
        // println!("primordials={:?}, anticipations={:?}, value={}", primordials, anticipations, value);

        Ok(Config { primordials , anticipations, value })
    }

}

pub fn run(config: Config) -> Result<f32, Box<dyn Error>> {
    let result = process(&config.primordials, &config.anticipations, config.value);
    Ok(result)
}


// 拉格朗日插值计算核心函数
pub fn process(primoridals: &[f32], anticipations: &[f32], currentValue: f32) -> f32 {
    let primoridalsCount = primoridals.len();
    if primoridalsCount != anticipations.len() {
        panic!("Error: primoridals count must be equal to anticipations count");
    }
    if currentValue <= 0.0 {
        panic!("Error: currentValue must gather than 0");
    }
    let mut result: f32 = 0.0;
    for i in 0..primoridalsCount {
        let mut temp: f32 = anticipations[i];
        for j in 0..primoridalsCount {
            if j != i {
                temp *= (currentValue - primoridals[j]) / (primoridals[i] - primoridals[j])
            }
        }
        result += temp;
    }
    result
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn convert_arg_str(arg: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    let temp: Vec<&str> = arg.split(",").collect();
    let temp: Vec<f32> = temp.into_iter().map(|i| 
        remove_whitespace(i).parse::<f32>().expect("convert arg to f32 fault")
    ).collect();
    Ok(temp)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample() {
        let xValues = [1.0, 2.0, 3.0, 4.0];
        let yValues = [1.0, 4.0, 9.0, 16.0];
        let value = 2.5;
    
        let result = process(&xValues, &yValues, value);
        assert_eq!(result, 6.25);
    }
    
}