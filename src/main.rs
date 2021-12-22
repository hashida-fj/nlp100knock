use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

mod section1;

pub fn count_file_lines(path: &str) -> std::io::Result<usize> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut counter = 0;
    br.lines().for_each(|_| counter += 1);
    Ok(counter)
}

pub fn tab_to_space(path: &str) -> std::io::Result<()> {
    let file = File::open(path)?;
    let cursor = BufReader::new(file);

    for line in cursor.lines() {
        println!("{}", line?.replace("\t", " "));
    }

    Ok(())
}

pub fn get_col(path: &str, out: &str, n: usize) -> std::io::Result<()> {
    let cursor = BufReader::new(File::open(path)?);
    let mut writer = BufWriter::new(File::create(out)?);

    for line in cursor.lines() {
        match line {
            Ok(it) => {
                let item = it.split_whitespace().collect::<Vec<&str>>()[n];
                writer.write_all(item.as_bytes())?;
                writer.write_all(b"\n")?;
            }
            Err(err) => return Err(err),
        };
    }
    Ok(())
}

pub fn paste(left: &str, right: &str) -> std::io::Result<()> {
    let cursorl = BufReader::new(File::open(left)?);
    let cursorr = BufReader::new(File::open(right)?);
    for pair in cursorl.lines().zip(cursorr.lines()) {
        match pair {
            (Ok(l), Ok(r)) => {
                println!("{}\t{}", l, r);
            }
            (Err(err), _) => return Err(err),
            (_, Err(err)) => return Err(err),
        }
    }
    Ok(())
}

pub fn head(path: &str, n: usize) -> std::io::Result<()> {
    let cursor = BufReader::new(File::open(path)?);
    for line in cursor.lines().take(n) {
        println!("{}", line?);
    }
    Ok(())
}

pub fn tail(path: &str, n: usize) -> std::io::Result<()> {
    let cursor = BufReader::new(File::open(path)?);
    cursor
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .into_iter()
        .rev()
        .take(n)
        .rev()
        .for_each(|line| {
            println!("{}", line);
        });
    Ok(())
}

pub fn split(path: &str, n: usize) -> std::io::Result<()> {
    let cursor = BufReader::new(File::open(path)?);

    let chunks = cursor
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let len = chunks.len();
    let div = len / n;
    let rest = len % n;

    println!("len:{}", len);
    println!("n  :{}", n);
    println!("div:{}", div);
    println!("mod:{}", rest);

    let chunk_size = if rest == 0 { div } else { div + 1 };

    for (i, chunk) in chunks.chunks(chunk_size).enumerate() {
        let file = File::create(format!("split{}.txt", i))?;
        let mut writer = BufWriter::new(file);

        for l in chunk {
            writer.write_all(l.as_bytes())?;
            writer.write_all(b"\n")?;
        }
    }

    Ok(())
}

pub fn count_uniq_words(path: &str) -> std::io::Result<usize> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let set: HashSet<_> = contents
        .lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .collect();

    Ok(set.len())
}

pub fn sort(path: &str, n: usize) -> std::io::Result<()> {
    let cursor = BufReader::new(File::open(path)?);
    let mut lines: Vec<_> = cursor.lines().map(|l| l.unwrap()).collect();

    lines.sort_by(|a, b| {
        a.split_whitespace()
            .nth(n)
            .cmp(&b.split_whitespace().nth(n))
    });

    for l in lines {
        println!("{}", l);
    }

    Ok(())
}

pub fn sort2(path: &str, n: usize) -> std::io::Result<()> {
    let cursor = BufReader::new(File::open(path)?);
    let mut lines = cursor.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    lines.sort_by(|a, b| {
        a.split_whitespace()
            .nth(n)
            .cmp(&b.split_whitespace().nth(n))
    });
    Ok(())
}

pub fn word_frequency(path: &str) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut result: HashMap<&str, usize> = HashMap::new();
    for line in contents.lines() {
        let name = line.split_whitespace().next().unwrap();
        result.insert(name, result.get(name).unwrap_or(&0) + 1);
    }

    let mut result = result.into_iter().collect::<Vec<_>>();
    result.sort_by(|a, b| b.1.cmp(&a.1));

    for r in result {
        println!("{} {}", r.0, r.1);
    }
    Ok(())
}

fn main() {
    // section1::run_section1();
    run_section2();
}

fn run_section2() {
    // println!("{:?}", count_file_lines("./assets/popular-names.txt"));
    // println!("{:?}", tab_to_space("./assets/popular-names.txt"));
    // println!("{:?}", get_col("./assets/popular-names.txt", "out1.txt", 1));
    // println!("{:?}", get_col("./assets/popular-names.txt", "out2.txt", 2));
    // println!("{:?}", paste("out1.txt", "out2.txt"));
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // println!(
    //     "{:?}",
    //     tail(
    //         "./assets/popular-names.txt",
    //         args.get(1).unwrap_or(&"5".to_string()).parse().unwrap_or(5)
    //     )
    // );

    //println!("{:?}", split("poem.txt", 5));
    //println!("{:?}", count_uniq_words("./assets/popular-names.txt"))
    //println!("{:?}", sort("./assets/popular-names.txt", 3))
    println!("{:?}", word_frequency("./assets/popular-names.txt"))
}
