extern crate cgmath;
extern crate clap;
extern crate ply_rs;

mod input;
mod output;
mod generate;

use clap::{App, Arg};
use cgmath::Vector3;

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

    let mesh = match input::read_ply_as_mesh(matches.value_of("INPUT").unwrap()) {
        Err(e) => panic!("Couldn't read PLY file:\n{}", e),
        Ok(mesh) => mesh,
    };
    print_mesh_stat(&mesh);

    let gen_mesh = generate::subdiv(mesh);
    print_mesh_stat(&gen_mesh);

    output::write_stl(matches.value_of("OUTPUT").unwrap(), &gen_mesh);
}

fn print_mesh_stat(mesh: &Vec<[Vector3<f64>; 3]>) {
    println!("Mesh: #tris={}", mesh.len());
}
