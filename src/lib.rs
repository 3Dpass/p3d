#![no_std]

use alloc::string::String;
use alloc::vec::Vec;

use obj::{load_obj, Obj, Vertex};
use tri_mesh::prelude::*;
use cgmath::Point2;

#[macro_use]
extern crate ndarray;

#[macro_use]
extern crate alloc;

use ndarray::arr2;
use ndarray::Array3;
use crate::algo_grid::get_contour;
use crate::contour::Rect;

mod polyline;
mod contour;
mod algo_grid;
use algo_grid::find_top_std;
type Vec2 = Point2<f64>;


#[derive(Debug)]
pub enum AlgoType {
    Grid2d,
    Spectr,
}

#[derive(Debug)]
pub struct P3DError {}


#[allow(unused_variables)]
//pub fn p3d_process<F>(scan_name: &PathBuf, algo: AlgoType, par1: i16, par2: i16, fptr: Option<F>) -> Result<Vec<String>, std::io::Error>
//    where F: Fn(i64, i64, String) -> i64
pub fn p3d_process(input: &[u8], algo: AlgoType, par1: i16, par2: i16 ) -> Result<Vec<String>, P3DError>
{
    let grid_size: i16 = par1;
    let n_sections: i16 = par2;

    let model: Obj<Vertex, u32> = load_obj(input).unwrap();

    let verts = model.vertices
        .iter()
        .flat_map(|v| v.position.iter())
        .map(|v| <f64 as cgmath::NumCast>::from(*v).unwrap())
        .collect();

    let mut mesh = MeshBuilder::new()
        .with_indices(model.indices)
        .with_positions(verts)
        .build().unwrap();

    let mut triangles: Array3<f64> = Array3::zeros((mesh.no_faces(), 3, 3));

    for (i, fid) in mesh.face_iter().enumerate() {
        let vs = mesh.face_vertices(fid);
        let v1 = mesh.vertex_position(vs.0);
        let v2 = mesh.vertex_position(vs.1);
        let v3 = mesh.vertex_position(vs.2);
        triangles.slice_mut(s![i, .., ..])
            .assign(
            &arr2(&[
                    [v1.x as f64, v1.y as f64, v1.z as f64],
                    [v2.x as f64, v2.y as f64, v2.z as f64],
                    [v3.x as f64, v3.y as f64, v3.z as f64],
                ]
            ));
    }

    let pit1 = algo_grid::principal_inertia_transform(triangles);

    let pit = pit1; //.t();

    let a: Matrix4<f64> = Matrix4::new(
        pit[[0,0]], pit[[0,1]], pit[[0,2]], pit[[0,3]],
        pit[[1,0]], pit[[1,1]], pit[[1,2]], pit[[1,3]],
        pit[[2,0]], pit[[2,1]], pit[[2,2]], pit[[2,3]],
        pit[[3,0]], pit[[3,1]], pit[[3,2]], pit[[3,3]],
    ); //.transpose();

    let a: Matrix3<f64> = Matrix3::new(
        pit[[0,0]], pit[[0,1]], pit[[0,2]],
        pit[[1,0]], pit[[1,1]], pit[[1,2]],
        pit[[2,0]], pit[[2,1]], pit[[2,2]],
    ); //.transpose();

    let b = a.invert().unwrap();

    let tr: Matrix4<f64> = Matrix4::new(
        b.x[0], b.x[1], b.x[2], 0.0,
        b.y[0], b.y[1], b.y[2], 0.0,
        b.z[0], b.z[1], b.z[2], 0.0,
        0.0, 0.0, 0.0, 1.0
    ); //.transpose();

    let shift = Vector3::new(pit[[0,3]], pit[[1,3]], pit[[2,3]]);

    mesh.translate(shift);
    mesh.apply_transformation(tr);

    let (mut min_x, mut max_x) = (f64::MAX, f64::MIN);
    let (mut min_y, mut max_y) = (f64::MAX, f64::MIN);
    let (mut min_z, mut max_z) = (f64::MAX, f64::MIN);

    for vid in mesh.vertex_iter() {
        let v = mesh.vertex_position(vid);
        if v.x < min_x { min_x = v.x; }
        if v.x > max_x { max_x = v.x; }
        if v.y < min_y { min_y = v.y; }
        if v.y > max_y { max_y = v.y; }
        if v.z < min_z { min_z = v.z; }
        if v.z > max_z { max_z = v.z; }
    }

    let depth = 10;
    let mut cntrs: Vec<Vec<Vec2>> = Vec::with_capacity(depth);
    let step = (max_z - min_z) / (1.0f64 + n_sections as f64);
    for n in 0..n_sections {
        let z_sect = min_z + (n as f64 + 1.0f64) * step;
        let cntr = get_contour(&mesh, z_sect);
        if cntr.len() > 0 {
            cntrs.push(cntr);
        }
    }
    let rect = Rect::new(min_x, max_x, min_y, max_y);

    let res = find_top_std(&cntrs, depth as usize, grid_size, rect);

    Ok(res)
}
