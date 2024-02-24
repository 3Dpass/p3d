use std::collections::HashSet;
use alloc::vec::Vec;
use peroxide::numerical::eigen::EigenMethod;
use peroxide::{fuga::*, c};
use cgmath::{MetricSpace, Vector3, Matrix3}; // impl distance2
use cgmath::Point2;

type Vec2 = Point2<f64>;

pub fn mass_and_principal(vertex_position: &Vec<Vector3<f64>>, indices:&Vec<u32>) -> (Vector3<f64>, Matrix3<f64>) {

    let mut integral_sum = [0f64; 10];

    let mut f1:Vector3<f64> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut f2:Vector3<f64> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut f3:Vector3<f64> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut g0:Vector3<f64> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut g1:Vector3<f64> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut g2:Vector3<f64> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut cross:Vector3<f64> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

    for i in (0..indices.len()).step_by(3) {
        let v1 = vertex_position[indices[i + 1] as usize];
        let v2 = vertex_position[indices[i + 2] as usize];
        let v3 = vertex_position[indices[i + 0] as usize];
        f1.x = v1.x + v2.x + v3.x;
        f1.y = v1.y + v2.y + v3.y;
        f1.z = v1.z + v2.z + v3.z;

        f2.x = v1.x * v1.x +
               v2.x * v2.x +
               v1.x * v2.x +
               v2.x * f1.x;

        f2.y = v1.y * v1.y +
               v2.y * v2.y +
               v1.y * v2.y +
               v2.y * f1.y;

        f2.z = v1.z * v1.z +
               v2.z * v2.z +
               v1.z * v2.z +
               v2.z * f1.z;

        f3.x = v1.x * v1.x * v1.x +
               v1.x * v1.x * v2.x +
               v1.x * v2.x * v2.x +
               v2.x * v2.x * v2.x +
               v3.x * f2.x;

        f3.y = v1.y * v1.y * v1.y +
               v1.y * v1.y * v2.y +
               v1.y * v2.y * v2.y +
               v2.y * v2.y * v2.y +
               v3.y * f2.y;

        f3.z = v1.z * v1.z * v1.z +
               v1.z * v1.z * v2.z +
               v1.z * v2.z * v2.z +
               v2.z * v2.z * v2.z +
               v3.z * f2.z;

        g0.x = f2.x + (v1.x + f1.x) * v1.x;
        g0.y = f2.y + (v1.y + f1.y) * v1.y;
        g0.z = f2.z + (v1.z + f1.z) * v1.z;

        g1.x = f2.x + (v2.x + f1.x) * v2.x;
        g1.y = f2.y + (v2.y + f1.y) * v2.y;
        g1.z = f2.z + (v2.z + f1.z) * v2.z;

        g2.x = f2.x + (v3.x + f1.x) * v3.x;
        g2.y = f2.y + (v3.y + f1.y) * v3.y;
        g2.z = f2.z + (v3.z + f1.z) * v3.z;

        let d1 = [
            v2.x - v1.x,
            v2.y - v1.y,
            v2.z - v1.z,
        ];

        let d2 = [
            v3.x - v2.x,
            v3.y - v2.y,
            v3.z - v2.z,
        ];

        cross.x = d1[1] * d2[2] - d1[2] * d2[1];
        cross.y = d1[2] * d2[0] - d1[0] * d2[2];
        cross.z = d1[0] * d2[1] - d1[1] * d2[0];

        integral_sum[0] += cross.x * f1.x;

        integral_sum[1] += cross.x * f2.x;
        integral_sum[2] += cross.y * f2.y;
        integral_sum[3] += cross.z * f2.z;

        integral_sum[4] += cross.x * f3.x;
        integral_sum[5] += cross.y * f3.y;
        integral_sum[6] += cross.z * f3.z;

        integral_sum[7] += cross.x * (v1.y * g0.x + v1.y * g1.x + v1.y * g2.x);
        integral_sum[8] += cross.y * (v1.z * g0.y + v1.z * g1.y + v1.z * g2.y);
        integral_sum[9] += cross.z * (v1.x * g0.z + v1.x * g1.z + v1.x * g2.z);
    }

    let mut integrated = [0f64; 10];
    const COEFFICIENTS:[f64;10] = [1./6., 1./24., 1./24., 1./24., 1./60., 1./60., 1./60., 1./120., 1./120., 1./120.];
    for j in 0..10 {
        integrated[j] = integral_sum[j] * COEFFICIENTS[j];
    }
    let volume = integrated[0];
    let mut center_mass = [0f64; 3];
    if volume.abs() >= 1e-10 {
        center_mass[0] = integrated[1] / volume;
        center_mass[1] = integrated[2] / volume;
        center_mass[2] = integrated[3] / volume;
    }

    let mut inertia = [[0f64; 3]; 3];

    inertia[0][0] = integrated[5] + integrated[6] - volume * (center_mass[1] * center_mass[1] + center_mass[2] * center_mass[2]);

    inertia[1][1] = integrated[4] + integrated[6] -
                    volume * (center_mass[0] * center_mass[0] + center_mass[2] * center_mass[2]);

    inertia[2][2] = integrated[4] + integrated[5] -
                    volume * (center_mass[0] * center_mass[0] + center_mass[1] * center_mass[1]);

    inertia[0][1] = integrated[7] -
                    volume * center_mass[0] * center_mass[1];

    inertia[1][2] = integrated[8] -
                    volume * center_mass[1] * center_mass[2];

    inertia[0][2] = integrated[9] -
                    volume * center_mass[0] * center_mass[2];

    inertia[2][0] = inertia[0][2];
    inertia[2][1] = inertia[1][2];
    inertia[1][0] = inertia[0][1];

    let m =  matrix(c!(
         1. * inertia[0][0], -1. * inertia[0][1], -1. * inertia[0][2],
        -1. * inertia[1][0],  1. * inertia[1][1], -1. * inertia[1][2],
        -1. * inertia[2][0], -1. * inertia[2][1],  1. * inertia[2][2]
    ), 3, 3, Row);

    let e = eigen(&m, EigenMethod::Jacobi);
    let (_, v) = e.extract();

    let tr: Matrix3<f64> = Matrix3::new(
        v[(0,0)], v[(0,1)], v[(0,2)],
        v[(1,0)], v[(1,1)], v[(1,2)],
        v[(2,0)], v[(2,1)], v[(2,2)],
    );

    let shift = Vector3::new(-center_mass[0], -center_mass[1], -center_mass[2]);

    (shift, tr)
}

pub fn intersect_2(verts: &Vec<Vector3<f64>>, indices: &Vec<u32>, z_sect: f64, delta: f64) -> Vec::<Vec2> {

    let mut sect = Vec::<Vec2>::with_capacity(verts.len());
    let mut processed = HashSet::<(u32, u32)>::with_capacity(indices.len());

    for i in (0..indices.len()).step_by(3) {
        let i0 = indices[i];
        let i1 = indices[i+1];
        let i2 = indices[i+2];

        for mut tuple in [(i0, i1), (i0, i2), (i1, i2)] {
            tuple = if tuple.0 > tuple.1 {(tuple.1, tuple.0)} else {tuple};
            if !processed.contains(&tuple) {
                processed.insert(tuple);
                let p1 = verts[tuple.0 as usize];
                let p2 = verts[tuple.1 as usize];
                if p2.z >= z_sect && p1.z <= z_sect || p2.z <= z_sect && p1.z >= z_sect {
                    let (x, y);
                    let z1 = z_sect - p1.z;
                    let z2 = p2.z - z_sect;
                    if z1.abs() < delta {
                        (x, y) = (p1.x, p1.y);
                    }
                    else if z2.abs() < delta {
                        (x, y) = (p2.x, p2.y);
                    }
                    else {
                        let k = z2 / z1;
                        x = (p2.x + k * p1.x) / (k + 1.0);
                        y = (p2.y + k * p1.y) / (k + 1.0);
                    }
                    sect.push(Vec2{x, y});
                }
            }
        }

    }

    sect

}

pub fn get_contour(sect: &mut Vec<Vec2>) {
    let len = sect.len();

    for i in 0..len-1 {
        let mut nearest_distance = u32::MAX;
        let mut j = 0;
        for k in i+1..len {
            let distance = (sect[i].distance2(sect[k]) as f32 * 10000.0) as u32;
            if distance < nearest_distance {
                nearest_distance = distance;
                j = k;
            } 
        }
        sect.swap(i + 1, j);
    }

    sect.push(sect.first().unwrap().clone());
}
