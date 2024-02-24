extern crate alloc;

use obj::ObjError;
use tri_mesh::mesh_builder::Error as MeshError;

mod grid2d;
mod grid2d_v3m;

#[derive(Debug)]
pub enum AlgoType {
    Grid2d,
    Grid2dV2,
    Grid2dV3,
    Grid2dV3a,
    Grid2dV3m,
    Spectr,
}

#[derive(Debug)]
pub enum P3DError {
    InvalidObject(ObjError),
    MeshError(MeshError),
    MathError,
    InvalidObjFile,
    UnsupportedAlgorithms,
    TooManySearches,
}

pub fn p3d_process(input: &[u8], algo: AlgoType, par1: i16, par2: i16, trans: Option<[u8;4]>) -> Result<Vec<String>, P3DError> {
    p3d_process_n(input, algo, 10, par1, par2, trans)
}

pub fn p3d_process_n(input: &[u8], algo: AlgoType, depth: usize, par1: i16, par2: i16, trans: Option<[u8;4]>) -> Result<Vec<String>, P3DError> {
    match algo {
        AlgoType::Grid2dV2 => grid2d::p3d_process_n(input, algo, depth, par1, par2, trans),
        AlgoType::Grid2dV3 => grid2d::p3d_process_n(input, algo, depth, par1, par2, trans),
        AlgoType::Grid2dV3a => grid2d::p3d_process_n(input, algo, depth, par1, par2, trans),
        AlgoType::Grid2dV3m => grid2d_v3m::p3d_process_n(input, algo, depth, par1, par2, trans),
        _ => Err(P3DError::UnsupportedAlgorithms),
    }
}