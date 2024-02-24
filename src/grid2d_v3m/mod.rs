use std::io::{BufRead, BufReader, Cursor};
use std::vec;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use cgmath::{Vector3, Point2, vec3};
use sha2::{Digest, Sha256};
use tri_mesh::prelude::*;
use crate::{AlgoType, P3DError};

mod algo_grid;
mod polyline;
mod rect;
use polyline::GenPolyLines;
use rect::Rect;

#[allow(unused_variables)]
pub fn p3d_process_n(input: &[u8], algo: AlgoType, depth: usize, grid_size: i16, n_sections: i16, trans: Option<[u8;4]>) -> Result<Vec<String>, P3DError> {
    if grid_size > 8 || n_sections > 12 {
        Err(P3DError::TooManySearches)
    } else {
        let (verts, indices/*, normal*/) = load_obj(input)?;
        let (mut verts, indices) = reorganize_obj(verts, indices)?;
        let (shift, tr) = algo_grid::mass_and_principal(&verts, &indices);
        let (v_min, v_max) = transform(&mut verts, shift, tr, trans);
        let result = find_top_std(verts, indices, v_min, v_max, depth, grid_size as usize, n_sections as usize);
        result
    }
}

fn load_obj(input: &[u8]) -> Result<(Vec<Vector3<f64>>, Vec<u32>/*, Vec<Vector3<f32>>*/), P3DError>{
	let mut verts  : Vec<Vector3<f32>> = vec![];
	let mut indices: Vec<u32>          = vec![];
	// let mut normal : Vec<Vector3<f32>> = vec![];
    let cursor = Cursor::new(input);
	let reader = BufReader::new(cursor);
	for line in reader.lines() {
        let line = line.map_err(|_| P3DError::InvalidObjFile)?;
        let words: Vec<&str> = line.split_whitespace().collect();
		if words.len() > 0 {
			match words[0] {
				"v" => {
                    if words.len() != 4 {
                        return Err(P3DError::InvalidObjFile);
                    }
					verts.push(Vector3::new(
						words[1].parse().map_err(|_| P3DError::InvalidObjFile)?,
						words[2].parse().map_err(|_| P3DError::InvalidObjFile)?,
						words[3].parse().map_err(|_| P3DError::InvalidObjFile)?));
				},
                "f" => {
                    if words.len() != 4 {
                        return Err(P3DError::InvalidObjFile);
                    }
                    for idx in 1..=3 {
					    let swords: Vec<&str> = words[idx].split("/").collect();
                        if swords.len() != 3 {return Err(P3DError::InvalidObjFile);}
                        let i:u32 = swords[0].parse().map_err(|_| P3DError::InvalidObjFile)?;
                        indices.push(i-1);
                    }
				},
				// "vn" => {
				// 	normal.push(Vector3::new(
				// 		words[1].parse().map_err(|_| P3DError::InvalidObject)?,
				// 		words[2].parse().map_err(|_| P3DError::InvalidObject)?,
				// 		words[3].parse().map_err(|_| P3DError::InvalidObject)?));
				// },
				_=>{},
			}
		}
    }

	let mut verts_f64 = Vec::with_capacity(verts.len());
	for v in &verts {
		verts_f64.push(Vector3 {
			x: <f64 as cgmath::NumCast>::from(v.x).unwrap(),
			y: <f64 as cgmath::NumCast>::from(v.y).unwrap(),
			z: <f64 as cgmath::NumCast>::from(v.z).unwrap(),
		});
	}

    Ok((verts_f64,indices/*,normal*/))
}

fn reorganize_obj(verts: Vec<Vector3<f64>>, indices: Vec<u32>) -> Result<(Vec<Vector3<f64>>, Vec<u32>), P3DError> {
    let mut out_vertices = vec![Vector3::new(0.0,0.0,0.0); verts.len()];
    let mut out_indices           = vec![0; indices.len()];
    let mut tmp           		= vec![u32::MAX; indices.len()];

    let mut offset = 0u32;
    for i in 0..indices.len() {
        let index = indices[i] as usize;
        if tmp[index] == u32::MAX {

            if offset as usize >= out_vertices.len() || index >= verts.len() {
                return Err(P3DError::InvalidObjFile);
            }

            out_vertices[offset as usize] = verts[index];
            tmp[index] = offset;
            offset=offset+1;
        }
        out_indices[i] = tmp[index];
    }

    out_vertices.truncate(offset as usize);
    Ok((out_vertices, out_indices))
}

fn transform(verts: &mut Vec<Vector3<f64>>, shift: Vector3<f64>, tr: Matrix3<f64>, trans: Option<[u8;4]>) -> (Vector3<f64>, Vector3<f64>){
    let mut v_min = vec3(core::f64::MAX, core::f64::MAX, core::f64::MAX);
    let mut v_max = vec3(core::f64::MIN, core::f64::MIN, core::f64::MIN);

    let m3 = 
    if let Some(rot) = trans {
        let k = 45.0 / 256.0;
        let axis_normalized = Vector3::new(
            rot[0] as f64 * k,
            rot[1] as f64 * k,
            rot[2] as f64 * k,
        ).normalize();
        Matrix3::from_axis_angle(axis_normalized, Deg(rot[3] as f64 * k * 360.0 / 256.0))
    } else {
        Matrix3::one()
    };

    for v in verts.iter_mut() {
        *v += shift;
        *v = tr[0] * v.x + tr[1] * v.y + tr[2] * v.z;
        *v = m3[0] * v.x + m3[1] * v.y + m3[2] * v.z;

        v_min.x = v_min.x.min(v.x);
        v_min.y = v_min.y.min(v.y);
        v_min.z = v_min.z.min(v.z);

        v_max.x = v_max.x.max(v.x);
        v_max.y = v_max.y.max(v.y);
        v_max.z = v_max.z.max(v.z);
    }

    (v_min, v_max)
}

fn find_top_std(verts: Vec<Vector3<f64>>, indices: Vec<u32>, v_min: Vector3<f64>, v_max: Vector3<f64>, depth: usize, grid_size: usize, n_sections: usize) -> Result<Vec<String>, P3DError> {

    const N: usize = 2;
    let step = (v_max.z - v_min.z) / (1.0f64 + n_sections as f64);
    let rect = Rect::new(v_min.x, v_max.x, v_min.y, v_max.y);

    let result:Vec<Result<Vec<(f64, Vec<Point2<u8>>)>, P3DError>> =
    (0..n_sections).into_par_iter().map(|n| {
        let z_sect = v_min.z + (n as f64 + 1.0f64) * step;

        let mut cntr = algo_grid::intersect_2(&verts, &indices, z_sect, step * 0.01);
            
        let ss: Result<Vec<Vec<Point2<u8>>>, P3DError> = GenPolyLines::select_top_all(&cntr, grid_size, rect.clone());
        if ss.is_err() {
            return Err(ss.err().unwrap());
        } else {

            algo_grid::get_contour(&mut cntr);

            let nodes = ss.ok().unwrap();
            let ds: Vec<f64> = nodes.par_iter().map(|nodes| {
                GenPolyLines::line2points_sco2(&cntr, &nodes)
            }).collect();

            let mut top_in_cntr_max_value = 0f64;
            let mut top_in_cntr_max_index = N+1;
            let mut top_in_cntr:Vec<(f64, Vec<Point2<u8>>)> = Vec::with_capacity(N);
            for item in ds.into_iter().zip(nodes.into_iter()) {
                // println!("{:?}\t{:?}", item.0, item.1);
                if let Some(_) = top_in_cntr.iter().find(|a: &&(f64, Vec<Point2<u8>>)| a.0 == item.0) {
                } else {
                    if top_in_cntr.len() < N {
                        if item.0 >= top_in_cntr_max_value {
                            top_in_cntr_max_value = item.0;
                            top_in_cntr_max_index = top_in_cntr.len();
                        }
                        top_in_cntr.push(item);
                    } else {
                        if top_in_cntr[top_in_cntr_max_index].0 > item.0 {
                            top_in_cntr[top_in_cntr_max_index] = item;
                            top_in_cntr_max_value = 0f64;
                            top_in_cntr_max_index = 0;
                            for i in 0..N {
                                if top_in_cntr[i].0 >= top_in_cntr_max_value {
                                    top_in_cntr_max_value = top_in_cntr[i].0;
                                    top_in_cntr_max_index = i;
                                }
                            }
                        }
                    }
                }
            }
            return Ok(top_in_cntr);
        }

    }).collect();

    let mut ss = Vec::with_capacity(result.len());
    for r in result {
        if r.is_err() {
            return Err(r.err().unwrap());
        } else {
            let vec1 = r.ok().unwrap();
            let mut vec2 = Vec::with_capacity(vec1.len());
            for t in vec1 {
                vec2.push((t.0, GenPolyLines::calc_hash(&t.1)));
            }
            ss.push(vec2);
        }
    }

    let mut best_totals: Vec<(f64, Vec<u8>)> = Vec::with_capacity(depth);

    let mut ff = |d: f64, hash: Vec<u8>| {
        if let Some(_) = best_totals.iter().find(|a| a.0 == d) {
            return
        }
        else {
            if best_totals.len() == depth {
                let m = best_totals.iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)|
                        a.0.partial_cmp(&b.0).unwrap_or(core::cmp::Ordering::Equal)
                    );

                if let Some((i, r)) = m {
                    if r.0 > d {
                        best_totals[i] = (d, hash);
                    }
                }
            } else {
                best_totals.push((d, hash));
            }
        }
    };

    let mut stack: Vec<usize> = vec![0; n_sections];

    loop {
        let mut sco = 0.;
        let mut h: Vec<u8> = Vec::new();
        for l in 0..n_sections {
            let k = stack[l];
            if k < ss[l].len() {
                sco += ss[l][k].0;
                h.extend(ss[l][k].1.clone());
            }
        }
        ff(sco, h);

        let mut j = 0;
        while j < n_sections {
            if ss[j].len() == 0 {
                j += 1;
                continue
            }
            if stack[j] < N - 1 {
                stack[j] += 1;
                break
            }
            stack[j] = 0;
            j += 1;
        }
        if j == n_sections {
            break
        }
    }

    let mut hashes = vec![];
    best_totals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for hash in best_totals.iter() {
        let mut hasher = Sha256::new();
        hasher.update(hash.1.as_slice());

        let mut a: Vec<u8> = vec![0; 32 * n_sections];
        let mut buf= a.as_mut();
        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_str(&hash, &mut buf).unwrap();

        hashes.push(hex_hash.to_string());
    }
    hashes.dedup();
    Ok(hashes)
}
