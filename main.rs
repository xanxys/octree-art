extern crate cgmath;
extern crate clap;
extern crate ply_rs;

mod input;
mod output;
mod generate;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Octree Art")
        .about("Generates printable octree art structure from 3D polygon models.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use (.ply)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file to use (binary .stl)")
                .required(true)
                .index(2),
        )
        .get_matches();

    match input::read_ply_as_mesh(matches.value_of("INPUT").unwrap()) {
        Err(e) => println!("Couldn't read PLY file:\n{}", e),
        Ok(mesh) => output::write_stl(matches.value_of("OUTPUT").unwrap(), generate::subdiv(mesh)),
    }
}
