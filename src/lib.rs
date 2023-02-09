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
use algo_grid::{
    find_top_std,
    find_top_std_2,
    find_top_std_3,
};
type Vec2 = Point2<f64>;


#[derive(Debug)]
pub enum AlgoType {
    Grid2d,
    Grid2dV2,
    Grid2dV3,
    Spectr,
}

#[derive(Debug)]
pub struct P3DError {}


#[allow(unused_variables)]
//pub fn p3d_process<F>(scan_name: &PathBuf, algo: AlgoType, par1: i16, par2: i16, fptr: Option<F>) -> Result<Vec<String>, std::io::Error>
//    where F: Fn(i64, i64, String) -> i64
pub fn p3d_process(input: &[u8], algo: AlgoType, par1: i16, par2: i16, trans: Option<[u8;4]>) -> Result<Vec<String>, P3DError>
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

    if let Some(rot) = trans {
        let v: Vec<f64> = rot[0..4].iter().map(|&r| (r as f64) * 45.0 / 256.0).collect();
        let axis: Vector3<f64> = Vector3::new(v[0], v[1], v[2]);
        mesh.apply_transformation(
            Mat4::from_axis_angle(
                axis.normalize(),
                Deg((v[3] as f64) * 360.0 / 256.0))
        );
    }
    let (v_min, v_max) = mesh.extreme_coordinates();

    let depth = 10;
    let mut cntrs: Vec<Vec<Vec2>> = Vec::with_capacity(depth);
    let step = (v_max.z - v_min.z) / (1.0f64 + n_sections as f64);
    for n in 0..n_sections {
        let z_sect = v_min.z + (n as f64 + 1.0f64) * step;
        let cntr = get_contour(&mesh, z_sect);
        if cntr.len() > 0 {
            cntrs.push(cntr);
        }
    }
    let rect = Rect::new(v_min.x, v_max.x, v_min.y, v_max.y);

    let res = match algo {
        AlgoType::Grid2dV2 => find_top_std_2(&cntrs, depth as usize, n_sections as usize, grid_size as usize, rect),
        AlgoType::Grid2dV3 => find_top_std_3(&cntrs, depth as usize, n_sections as usize, grid_size as usize, rect),
        _ => find_top_std(&cntrs, depth as usize, grid_size, rect),
    };

    Ok(res)
}
