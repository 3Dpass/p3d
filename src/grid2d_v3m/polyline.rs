use alloc::vec::Vec;
use cgmath::point2;
use sha2::{Sha256, Digest};
use cgmath::Point2;

use crate::P3DError;
use super::rect::Rect;

type Vec2 = Point2<f64>;
type NearPointsGroup = [[u64; GRID_SIZE_MAX]; GRID_SIZE_MAX];

#[derive(Clone, Copy, Debug)]
pub(crate) struct LoopData {
    pub(crate) neibs: u64,
    pub(crate) map: u64,
    pub(crate) pos: Point2<u8>,
}

pub(crate) struct GenPolyLines {
}

impl GenPolyLines {

	pub(crate) fn line_zone(n:i32, rect: &Rect, points:&Vec<Point2<f64>>) -> u64 {
		fn chk_add(z: &mut u64, n: i32, i: i32, j: i32) {
			if 0 <= i && i < n && 0 <= j && j < n {
				*z |= GenPolyLines::point_to_line(i,j);
			}
		}

		let w = rect.width();
		let h = rect.height();

		let grid_dx: f64 = w / n as f64;
		let grid_dy: f64 = h / n as f64;

		let dx = 0.1 * grid_dx;
		let dy = 0.1 * grid_dy;

		let _len = points.len();

		let mut z: u64 = 0;

		for pp in points.iter() {
			let mut i = ((pp.x - rect.min_x()) / grid_dx) as i32;
			let mut j = ((pp.y - rect.min_y()) / grid_dy) as i32;

			if i == n { i = n - 1; }
			if j == n { j = n - 1; }

			z |= GenPolyLines::point_to_line(i,j);

			let xx = pp.x.round() - pp.x;
			let yy = pp.y.round() - pp.y;

			if xx.abs() < dx {
				let tmp_i = if xx >= 0.0 { i + 1 } else { i - 1 };
				chk_add(&mut z, n, tmp_i, j);
			}
			if yy.abs() < dy {
				let tmp_j = if yy >= 0.0 { j + 1 } else { j - 1 };
				chk_add(&mut z, n, i, tmp_j);
			}

			if yy.abs() < dy && xx.abs() < dx {
				let tmp_i = if xx >= 0.0 { i + 1 } else { i - 1 };
				let tmp_j = if yy >= 0.0 { j + 1 } else { j - 1 };
				chk_add(&mut z, n, tmp_i, tmp_j);
			}
		}

		z
	}

    pub (crate) fn calc_hash(nodes:&Vec<Point2<u8>>) -> Vec<u8> {
        let data: Vec<u8> = nodes.as_slice().iter()
            .flat_map(|&p| [(p.x as i32).to_be_bytes(), (p.y as i32).to_be_bytes()])
            .flatten()
            .collect();

        let mut hasher = Sha256::new();
        hasher.update(data.as_slice());

        let hash = hasher.finalize();
        hash.to_vec()
    }

    pub (crate) fn line2points_sco2(cn: &Vec<Point2<f64>>, nodes: &[Point2<u8>]) -> f64 {
        let mut l: f64 = 0.0;
        let mut p1 = point2(0.5, 0.5);

        let mut ll: Vec<(Point2<f64>, f64)> = Vec::with_capacity(nodes.len());
        ll.push((p1, l));

        for i in 1..nodes.len() {
            let p2 = point2(nodes[i].x as f64 + 0.5, nodes[i].y as f64 + 0.5);
            l = l + ((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y));
            ll.push((p2, l));
            p1 = p2;
        }

        let dl = l / cn.len() as f64;
        let mut m = 1;
        let mut p = ll[0].0;
        let mut s1 = ll[0];
        let mut s2 = ll[1];

        let mut sum = (cn[0].x - 0.5) * (cn[0].x - 0.5) + (cn[0].y - 0.5) * (cn[0].y - 0.5);

        for k in 1..cn.len() {
            let r: f64 = (k as f64) * dl;
            while m < ll.len() && r >= s2.1  {
                p = s2.0;
                m += 1;
                s1 = s2;
                s2 = ll[m];
            }

            let dd = r - s1.1;
            let (dx, dy): (f64, f64) =
                if (s2.0.x - s1.0.x).abs() > 1.0e-10 {
                    let kk = (s2.0.y - s1.0.y) / (s2.0.x - s1.0.x);
                    let dx = dd / (1.0 + kk*kk).sqrt();
                    let dy = kk * dx;
                    (p.x + dx, p.y + dy)
                }
                else {
                    let dx = 0.0;
                    let dy = dd;
                    (p.x + dx, p.y + dy)
                };

            sum += (cn[k].x - dx) * (cn[k].x - dx) + (cn[k].y - dy) * (cn[k].y - dy);
        }

        sum / cn.len() as f64
    }

    pub (crate) fn select_top_all(cntr: &Vec<Vec2>, grid_size: usize, rect: Rect) -> Result<Vec<Vec<Point2<u8>>>, P3DError> {

        let mut near_points = [[0u64; GRID_SIZE_MAX]; GRID_SIZE_MAX];
        let mut result: Vec<Vec<Point2<u8>>> = Vec::with_capacity(NODES_MAX);

        let zone = GenPolyLines::line_zone(grid_size as i32, &rect, &cntr) | 1;
        GenPolyLines::search_near_points(grid_size as i32, zone, &mut near_points);

        let start_point_near = near_points[0][0];
        let b =
        if start_point_near.count_ones() < 2
        || start_point_near == 1029
        || start_point_near == 263169
        || start_point_near == 393217
        || start_point_near == 196609
        { false }
        else { true };

        if b {

            let start_point = Point2 { x: 0, y: 0 };
            let mut line_buf_nodes = [point2(0, 0); 17];
            let mut loop_array = [LoopData { map:0, neibs:0, pos:point2(0, 0) }; 17];
            let mut search_counter = 0;

            line_buf_nodes[0] = start_point;

            let mut loop_array_cursor = 0;
            GenPolyLines::add_loop_data(point2(0,0), &near_points[0][0], loop_array_cursor+1, 0, BAKE_MAP[0][0], &mut loop_array[loop_array_cursor]);

            loop {

                search_counter = search_counter+1;
                if search_counter > SEARCHES_MAX {
                    return Err(P3DError::TooManySearches); 
                }

                let allow = if loop_array_cursor>0 { loop_array[loop_array_cursor-1].map & 459780 != 459780 } else { true };

                let mut data = &mut loop_array[loop_array_cursor];

                // debug
                // println!("point: {:?}, neibs: {:?}", data.pos, data.neibs);

                let p = GenPolyLines::get_next_neib(&mut data, grid_size as i32);

                if !p.is_none() && (p.unwrap() == start_point || allow) {
                    let p = p.unwrap();
                    let next_loop_array_index = loop_array_cursor+1;
                    line_buf_nodes[next_loop_array_index] = p;

                    if p == start_point {
                        result.push(line_buf_nodes[..=next_loop_array_index].to_vec());
                        if result.len() >= NODES_MAX {
                            return Err(P3DError::TooManySearches);
                        }
                    } else {
                        loop_array_cursor = next_loop_array_index;
                        GenPolyLines::add_loop_data(p, &near_points[p.x as usize][p.y as usize], loop_array_cursor+1, data.map, BAKE_MAP[p.x as usize][p.y as usize], &mut loop_array[loop_array_cursor]);
                    }
                } else {
                    if loop_array_cursor > 0 {
                        loop_array_cursor = loop_array_cursor - 1;
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(result)
    }

    fn add_loop_data(pos:Point2<u8>, near_points:&u64, len_line_buf_nodes:usize, parent_map:u64, map:u64, loop_data:&mut LoopData) {
        loop_data.pos = pos;
        loop_data.map = parent_map | map;
        loop_data.neibs = near_points & !loop_data.map;
        if len_line_buf_nodes<=5 {
            loop_data.neibs &= 18446744073709551614;
        } else {
            loop_data.neibs |= 1;
        }
    }

    fn get_next_neib(data:&mut LoopData, grid_size:i32) -> Option<Point2<u8>> {
        for (i, j) in BOUNDS {
            let x = i + (data.pos.x as i32);
            let y = j + (data.pos.y as i32);
            if x >= 0 && x < grid_size && y >= 0 && y < grid_size {
                let bitpos = GenPolyLines::point_to_line(x,y);
                if data.neibs & bitpos != 0 {
                    data.neibs ^= bitpos;
                    return Some(point2(x as u8, y as u8));
                }
            }
        }
        return Option::None;
    }

    fn near_points(grid_size: i32, z:u64, neibs:&mut NearPointsGroup, start_point:(i32,i32)) {
        // let bounds = [
        //     (start_point.0 - 2, start_point.1 - 2), 
        //     (start_point.0 - 1, start_point.1 - 2), 
        //     (start_point.0    , start_point.1 - 2), 
        //     (start_point.0 + 1, start_point.1 - 2), 
        //     (start_point.0 + 2, start_point.1 - 2), 

        //     (start_point.0 + 2, start_point.1 - 1), 
        //     (start_point.0 + 2, start_point.1), 
        //     (start_point.0 + 2, start_point.1 + 1), 

        //     (start_point.0 - 2, start_point.1 + 2), 
        //     (start_point.0 - 1, start_point.1 + 2), 
        //     (start_point.0    , start_point.1 + 2), 
        //     (start_point.0 + 1, start_point.1 + 2), 
        //     (start_point.0 + 2, start_point.1 + 2), 

        //     (start_point.0 - 2, start_point.1 - 1), 
        //     (start_point.0 - 2, start_point.1), 
        //     (start_point.0 - 2, start_point.1 + 1), 
        //     ];

        let mut neib = 0u64;

        for (i, j) in BOUNDS {
            let x = i + start_point.0;
            let y = j + start_point.1;
            if (x >= 0 && x < grid_size && y >= 0 && y < grid_size) && (z & GenPolyLines::point_to_line(x,y) != 0) {
                neib |= GenPolyLines::point_to_line(x,y);
            }
        }

        neibs[start_point.0 as usize][start_point.1 as usize] = neib;

    }

    fn search_near_points(grid_size: i32, zone: u64, near_points: &mut NearPointsGroup) {
        for x in 0..grid_size {
            for y in 0..grid_size {
                if zone & GenPolyLines::point_to_line(x,y) != 0 {
                    GenPolyLines::near_points(grid_size, zone, near_points, (x,y));
                }
            }
        }
    }

    fn point_to_line(x: i32, y: i32) -> u64 {
        1u64 << y * GRID_SIZE_MAX as i32 + x
    }

}

const BAKE_MAP:[[u64;GRID_SIZE_MAX];GRID_SIZE_MAX] = [
    [771, 197379, 50529024, 12935430144, 3311470116864, 847736349917184, 217020505578799104, 217017207043915776],
    [1799, 460551, 117901056, 30182670336, 7726763606016, 1978051483140096, 506381179683864576, 506373483102470144],
    [3598, 921102, 235802112, 60365340672, 15453527212032, 3956102966280192, 1012762359367729152, 1012746966204940288],
    [7196, 1842204, 471604224, 120730681344, 30907054424064, 7912205932560384, 2025524718735458304, 2025493932409880576],
    [14392, 3684408, 943208448, 241461362688, 61814108848128, 15824411865120768, 4051049437470916608, 4050987864819761152],
    [28784, 7368816, 1886416896, 482922725376, 123628217696256, 31648823730241536, 8102098874941833216, 8101975729639522304],
    [57568, 14737632, 3772833792, 965845450752, 247256435392512, 63297647460483072, 16204197749883666432, 16203951459279044608],
    [49344, 12632256, 3233857536, 827867529216, 211934087479296, 54255126394699776, 13889312357043142656, 13889101250810609664]
];

const BOUNDS:[(i32,i32);16] = [
    (-2, -2), 
    (-1, -2), 
    ( 0, -2), 
    ( 1, -2), 
    ( 2, -2), 

    ( 2, -1), 
    ( 2,  0), 
    ( 2,  1), 

    (-2,  2), 
    (-1,  2), 
    ( 0,  2), 
    ( 1,  2), 
    ( 2,  2), 

    (-2, -1), 
    (-2,  0), 
    (-2,  1), 
];

const SEARCHES_MAX:usize = 10_000_000;
const NODES_MAX:usize = 1_000_000;

const GRID_SIZE_MAX:usize = 8;