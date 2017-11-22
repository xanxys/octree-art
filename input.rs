use std::io;
use ply_rs as ply;
use ply_rs::ply::Property;
use cgmath::Vector3;
use std::fs::File;
use std::str::FromStr;
use std::io::Read;

pub fn read_ply_as_mesh(path: &str) -> io::Result<Vec<[Vector3<f64>; 3]>> {
    let mut f = File::open(path).unwrap();

    // Preprocess file, because ply-rs cannot handle exponent numbers like "1.0e-4"
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Err(e) => return Err(e),
        _ => {},
    }
    let contents_fixed = fix_all(&contents);

    let parser = ply::parser::Parser::<ply::ply::DefaultElement>::new();
    let ply = match parser.read_ply(&mut contents_fixed.as_bytes()) {
        Err(e) => return Err(e),
        Ok(ply) => ply,
    };

    let mut mesh = Vec::with_capacity(ply.payload["face"].len());
    let ref vertices = &ply.payload["vertex"];
    for ref face in &ply.payload["face"] {
        let ref ixs = match &face["vertex_indices"] {   
            &Property::ListInt(ref ixs) => ixs,
            _ => continue,
        };

        let mut vs: [Vector3<f64>; 3] = [Vector3::new(0.0, 0.0, 0.0); 3];
        for i in 0..3 {
            let ref vertex = &vertices[ixs[i] as usize];
            match (&vertex["x"], &vertex["y"], &vertex["z"]) {
                (&Property::Float(ref x), &Property::Float(ref y), &Property::Float(ref z)) => {
                    vs[i] = Vector3::new(*x as f64, *y as f64, *z as f64);
                },
                _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Broken face / vertex")),
            }
        }
        mesh.push(vs);
    }
    Ok(mesh)
}

fn fix_all(t: &str) -> String {
    let mut lines: Vec<String> = vec![];
    for line in t.lines() {
        lines.push(fix_line(line));
    }
    lines.join("\n")
}

fn fix_line(t: &str) -> String {
    let mut words: Vec<String> = vec![];
    for word in t.split_whitespace() {
        words.push(match f64::from_str(word) {
            Ok(v) if word.contains("e") => format!("{:.}", v),
            _ => String::from(word),
        });
    }
    words.join(" ")
}
