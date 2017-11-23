use std::default::Default;
use std::f64;
use cgmath::Vector3;
use cgmath::InnerSpace;

type Tri = [Vector3<f64>; 3];
type V3 = Vector3<f64>;

/// (org, size). size >0.
/// spans [org, org + (size, size, size)]
#[derive(Clone, Copy, Debug)]
struct Cube(V3, f64);

/// pos, normal
#[derive(Clone, Copy, Debug)]
struct TriN(Tri, V3);


pub fn gen_octree_art(mesh: Vec<Tri>) -> Vec<Tri> {
    let mesh_n: Vec<TriN> = mesh.iter()
        .map(|&tri| {
            TriN(tri, (tri[1] - tri[0]).cross(tri[2] - tri[0]).normalize())
        })
        .collect();

    let (pmin, pmax) = aabb_for_mesh(mesh);
    let p_size = pmax - pmin;
    let cube = Cube(pmin, max(p_size.x, p_size.y, p_size.z));
    gen_printable(&divide(cube, &mesh_n, 2))
}

fn aabb_for_mesh(mesh: Vec<Tri>) -> (V3, V3) {
    let mut pmin = V3::new(f64::MAX, f64::MAX, f64::MAX);
    let mut pmax = V3::new(f64::MIN, f64::MIN, f64::MIN);
    for tri in mesh {
        for p in tri.into_iter() {
            pmin = min_v3(pmin, *p);
            pmax = max_v3(pmax, *p);
        }
    }
    (pmin, pmax)
}

// Artistic / mechanical: generate mesh based on octree and arbitrary params.
fn gen_printable(octree: &Octree<bool>) -> Vec<Tri> {
    vec![]
}


// Pure geometry: Octree division
#[derive(Clone)]
enum Octree<A> {
    Br(Box<[Octree<A>; 8]>),
    Leaf(A),
}

impl<A: Default> Default for Octree<A> {
    fn default() -> Octree<A> {
        Octree::Leaf(Default::default())
    }
}

/// Divide space, so that false: definitely empty cell. true: contains surface.
/// max_level = 0: Can only return Leaf. level=1 Br+Leaf....
fn divide(cube: Cube, tris: &Vec<TriN>, remaining_level: u32) -> Octree<bool> {
    println!(
        "divide: {:?} tris={} lv={}",
        cube.1,
        tris.len(),
        remaining_level
    );
    if remaining_level == 0 {
        Octree::Leaf(tris.iter().any(|&tri| intersect_cube_tri(&cube, &tri)))
    } else {
        let relevant_tris: Vec<TriN> = tris.iter()
            .filter(|tri| intersect_cube_tri(&cube, &tri))
            .map(|&tri| tri.clone())
            .collect();
        if relevant_tris.len() == 0 {
            Octree::Leaf(false)
        } else {
            let children_cubes = divide_cube(cube);
            let mut children_cells: [Octree<bool>; 8] = Default::default();
            for ix in 0..8 {
                children_cells[ix] =
                    divide(children_cubes[ix], &relevant_tris, remaining_level - 1);
            }
            Octree::Br(Box::new(children_cells))
        }
    }
}

fn divide_cube(Cube(org, size): Cube) -> [Cube; 8] {
    let ns = size * 0.5;
    let mut cubes = [Cube(org, size); 8];
    for i in 0..8 {
        let dpos = V3::new(
            if i & 1 > 0 { ns } else { 0.0 },
            if i & 2 > 0 { ns } else { 0.0 },
            if i & 4 > 0 { ns } else { 0.0 },
        );
        cubes[i] = Cube(org + dpos, ns);
    }
    cubes
}

fn intersect_cube_tri(&Cube(ref org, ref size): &Cube, &TriN(ref tri, ref n): &TriN) -> bool {
    intersect_cube0_tri(*size, &TriN([tri[0] - org, tri[1] - org, tri[2] - org], *n))
}

// Using separation axis theorem, we need to check XYZ + normal axes.
// XYZ check is equivalent to two Cubes intersection.
fn intersect_cube0_tri(cs: f64, &TriN(ref tri, ref n): &TriN) -> bool {
    // XYZ check.
    let tri_min = min_v3(tri[0], min_v3(tri[1], tri[2]));
    let tri_max = max_v3(tri[0], max_v3(tri[1], tri[2]));
    if intersect_iv(Iv(0.0, cs), Iv(tri_min.x, tri_max.x))
        || intersect_iv(Iv(0.0, cs), Iv(tri_min.y, tri_max.y))
        || intersect_iv(Iv(0.0, cs), Iv(tri_min.z, tri_max.z))
    {
        return true;
    }
    // normal axis check.
    let cube_dv = Vector3::new(cs * n.x, cs * n.y, cs * n.z);
    let cube_min = -(cube_dv.x.min(0.0) + cube_dv.y.min(0.0) + cube_dv.z.min(0.0));
    let cube_max = cube_dv.x.max(0.0) + cube_dv.y.max(0.0) + cube_dv.z.max(0.0);
    let tri_v = tri[0].dot(*n);
    if cube_min <= tri_v && tri_v <= cube_max {
        return true;
    }
    false
}

fn min_v3(a: V3, b: V3) -> V3 {
    Vector3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
}

fn max_v3(a: V3, b: V3) -> V3 {
    Vector3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
}


fn min(a: f64, b: f64, c: f64) -> f64 {
    a.min(b.min(c))
}

fn max(a: f64, b: f64, c: f64) -> f64 {
    a.max(b.max(c))
}

struct Iv(f64, f64);

fn intersect_iv(Iv(al, ah): Iv, Iv(bl, bh): Iv) -> bool {
    al.max(bl) >= ah.min(bh)
}
