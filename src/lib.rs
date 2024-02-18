use rand::Rng;
use std::{env, fs::read_to_string, fs::OpenOptions, io::Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    CommandLine,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error!")
    }
}

pub fn run() -> Result<()> {
    let limit = 1000000000;
    //let limit = 10000;
    let args: Vec<String> = env::args().collect();
    let file = args.last().ok_or(Error::CommandLine)?;
    let cities = read_to_string(file)?;
    let mut result = vec![];
    let mut rng = rand::thread_rng();

    let mut count = 0;

    'outer: while count <= limit {
        result = vec![];
        for city in cities.split('\n') {
            let temp = rng.gen_range(-40.0..60.0);
            let res = format!("{city};{:.2}\n", temp);

            result.push(res);

            count += 1;

            if count >= limit {
                let _ = write_results(&result);
                break 'outer;
            }
        }
        let _ = write_results(&result);
        println!("count = {count}");
    }

    println!("res count {}", result.len());

    Ok(())
}

fn write_results(result: &[String]) -> Result<()> {
    println!("WR");
    let mut file = OpenOptions::new().append(true).create(true).read(true).write(true).open("result.csv")?;

    for res in result {
        file.write_all(res.as_bytes())?;
    }

    let _ = file.flush();

    Ok(())
}
