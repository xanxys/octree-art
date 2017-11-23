use cgmath::Vector3;
use std::io;
use std::fs::OpenOptions;
use cgmath::InnerSpace;
use stl_io;

type V3 = Vector3<f64>;

pub fn write_stl(path: &str, mesh: &Vec<[V3; 3]>) -> io::Result<()> {
    let mut file = match OpenOptions::new().write(true).create(true).open(path) {
        Err(e) => return Err(e),
        Ok(f) => f,
    };

    let stl_mesh: Vec<stl_io::Triangle> = mesh.iter().map(|&tri| convert_tri(&tri)).collect();
    stl_io::write_stl(&mut file, stl_mesh.iter())
}

fn convert_tri(tri: &[V3; 3]) -> stl_io::Triangle {
    let n = (tri[1] - tri[0]).cross(tri[2] - tri[1]).normalize();
    stl_io::Triangle {
        normal: convert_v(&n),
        vertices: [convert_v(&tri[0]), convert_v(&tri[1]), convert_v(&tri[2])],
    }
}

fn convert_v(v: &V3) -> [f32; 3] {
    [v.x as f32, v.y as f32, v.z as f32]
}
