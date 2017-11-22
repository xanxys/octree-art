use std::io;
use ply_rs as ply;
use cgmath::Vector3;
use std::fs::File;
use std::str::FromStr;
use std::io::Read;

pub fn read_ply_as_mesh(path: &str) -> io::Result<Vec<[Vector3<f64>; 3]>> {
    let mut f = File::open(path).unwrap();
    
    // Preprocess file, because ply-rs cannot handle exponent numbers like "1.0e-4"
    let mut contents = String::new();
    f.read_to_string(&mut contents);
    let contents_fixed = fix_all(&contents);

    let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();
    let ply = p.read_ply(&mut contents_fixed.as_bytes());

    if ply.is_ok() {
        println!("Loaded {:#?}", ply.unwrap().header.elements);
    } else {
        println!("Not ok {:#?}", ply);
    }

    return Ok(vec![]);
}

fn fix_all(t: &str) -> String {
    let mut lines: Vec<String> = vec![];
    for line in t.lines() {
        lines.push(fix_line(line));
    }
    return lines.join("\n");
}

fn fix_line(t: &str) -> String {
    let mut words: Vec<String> = vec![];
    for word in t.split_whitespace() {
        words.push(
            match f64::from_str(word) {
                Ok(v) if word.contains("e") => format!("{:.}", v),
                _ => String::from(word),
            });
    }
    return words.join(" ");
}
