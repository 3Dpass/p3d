use alloc::collections::vec_deque::VecDeque;
use alloc::vec::IntoIter;
use alloc::vec::Vec;

use cgmath::MetricSpace;
#[allow(unused_imports)]
use cgmath::num_traits::Float;
use cgmath::Point2;
use sha2::{Digest, Sha256};

use super::contour::{CellSet, Cntr, Rect};

pub(crate) const DISTANCE: i32 = 2;

type Vec2 = Point2<f64>;

#[derive(Clone)]
pub(crate) struct PolyLine {
    pub(crate) nodes: Vec<Point2<i32>>,
    pub(crate) grid_size: i16,
}

impl<'a> PolyLine {
    pub(crate) fn new(pts: Vec<Point2<i32>>, grid_size: i16) -> Self {
        Self {
            nodes: pts,
            grid_size,
        }
    }

    fn line2points(
        &self,
        n: usize,
        rect: &Rect,
    ) -> Cntr {
        let mut l: f64 = 0.0;

        let mut res: Cntr = Cntr::new(None, self.grid_size, rect);
        let line2: Cntr = Cntr::new(
            Some(self.nodes.iter().map(|v| Point2::new(v.x as f64 + 0.5, v.y as f64 + 0.5)).collect()),
            self.grid_size,
            rect,
        );

        let mut p1 = line2.points[0];
        let mut ll: Vec<(Point2<f64>, f64)> = vec![(p1, 0.0)];

        for p2 in line2.points[1..].iter() {
            // TODO: check distance2
            l = l + p1.distance2(*p2);
            ll.push((*p2, l));
            p1 = *p2;
        }

        let tot_len = ll.last().unwrap().1;
        let dl = tot_len / n as f64;
        let mut m = 0;
        let mut p = ll[0].0;

        res.push(ll[0].0);

        for k in 1..n {
            let r: f64 = (k as f64) * dl;
            while m < ll.len() {
                let l = ll[m].1;
                if r < l {
                    // cur_path = r;
                    break;
                }
                p = ll[m].0;
                m += 1;
            }

            let s1 = ll[m - 1];
            let s2 = ll[m];
            //     # px = (p2[0] - p1[0]) / l * dl # TODO!!!
            //     # py = (p2[1] - p1[1]) / l * dl # TODO!!!

            let dd = r - s1.1;
            //let (mut dx, mut dy): (f64, f64) = (0.0, 0.0);
            let (dx, dy): (f64, f64) =
                if (s2.0.x - s1.0.x).abs() > 1.0e-10 {
                    let kk = (s2.0.y - s1.0.y) / (s2.0.x - s1.0.x);
                    let dx = dd / (1.0 + kk * kk).sqrt();
                    let dy = kk * dx;
                    (dx, dy)
                } else {
                    let dx = 0.0;
                    let dy = dd;
                    (dx, dy)
                };

            res.push(Point2 { x: p.x + dx, y: p.y + dy });
        }
        res
    }

    pub(crate) fn calc_hash(&self) -> Vec<u8> {
        let data: Vec<u8> = self.nodes.as_slice().iter()
            .flat_map(|&p| [p.x.to_be_bytes(), p.y.to_be_bytes()])
            .flatten()
            .collect();

        let mut hasher = Sha256::new();
        hasher.update(data.as_slice());

        let hash = hasher.finalize();
        hash.to_vec()
    }
}


pub(crate) struct GenPolyLines {
    cells: CellSet,
    line_buf: PolyLine,
    lev: i32,
}

impl GenPolyLines {
    pub(crate) fn new(z: CellSet, grid_size: i16) -> Self {
        Self {
            cells: z,
            line_buf: PolyLine::new(Vec::with_capacity(100), grid_size),
            lev: 0,
        }
    }

    // Function to calculate the squared centroid distance between two sets of points
    fn sco2(v1: &Cntr, v2: &Cntr) -> f64 {
        // Initialize squared sum as a floating point number
        let mut s = 0f64;

        // Iterate through the corresponding pairs of points in both input vectors
        for (a1, a2) in v1.points.iter().zip(v2.points.iter()) {
            // Calculate the squared Euclidean distance between current pair of points
            // and add it to the cumulative squared sum
            s += (a2.x - a1.x) * (a2.x - a1.x) + (a2.y - a1.y) * (a2.y - a1.y);
        }

        // Return the mean squared distance by dividing the cumulative squared sum
        // by the number of points
        s / (v1.points.len() as f64)
    }

    pub(crate) fn select_top(counters: &Vec<Vec<Vec2>>, n: usize, grid_size: i16, rect: Rect) -> Vec<(f64, PolyLine)> {
        let mut top_heap: VecDeque<(f64, PolyLine)> = VecDeque::with_capacity(n);
        // TODO: select start point from self.cells

        for cntr in counters.iter() {
            let cn = Cntr::new(Some(cntr.to_vec()), grid_size, &rect);
            let zone = cn.line_zone();

            let mut gen_lines = GenPolyLines::new(zone, grid_size);
            let start_point = Point2 { x: 0, y: 0 };
            gen_lines.line_buf.nodes.push(start_point);

            let cntr_size = cn.points.len();
            let calc_sco = |pl: &PolyLine|
                GenPolyLines::sco2(
                    &cn, &pl.line2points(cntr_size, &rect),
                );

            let mut ff = |pl: &PolyLine| {
                let d = calc_sco(pl);
                let len = top_heap.len();
                if len > 0 {
                    if d < top_heap.get(len - 1).unwrap().0 || len <= n {
                        if len == n {
                            top_heap.pop_front();
                        }
                        top_heap.push_back((d, pl.clone()));
                        top_heap.make_contiguous().sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                    }
                } else {
                    top_heap.push_back((d, pl.clone()));
                }
            };
            gen_lines.complete_line(&mut ff);
        }
        let v = top_heap.iter().cloned().collect();
        v
    }

    // This function selects the top n ranked PolyLines for each contour in a given grid.
    // The ranking is based on the score calculated by the `GenPolyLines::sco2` method.
    pub(crate) fn select_top_all(counters: &Vec<Vec<Vec2>>, n: usize, grid_size: usize, rect: Rect) -> Vec<Vec<(f64, Vec<u8>)>> {
        // Initialize a 2D Vector to store the top n PolyLines for each contour.
        let mut top_heap: Vec<Vec<(f64, Vec<u8>)>> = Vec::with_capacity(grid_size as usize);

        // Iterate through each contour.
        for cntr in counters.iter() {
            // Create a Deque to store the top n PolyLines within the current contour.
            let mut top_in_cntr: VecDeque<(f64, PolyLine)> = VecDeque::with_capacity(n);

            // Create Cntr and Zone objects needed for the calculations.
            let cn = Cntr::new(Some(cntr.to_vec()), grid_size as i16, &rect);
            let zone = cn.line_zone();

            // Initialize a new GenPolyLines object with the given zone.
            let mut gen_lines = GenPolyLines::new(zone, grid_size as i16);
            let start_point = Point2 { x: 0, y: 0 };
            gen_lines.line_buf.nodes.push(start_point);

            // Calculate the size of the current contour.
            let cntr_size = cn.points.len();

            // Closure to calculate the score for a given PolyLine using `GenPolyLines::sco2`.
            let calc_sco = |pl: &PolyLine|
                GenPolyLines::sco2(
                    &cn, &pl.line2points(cntr_size, &rect),
                );

            // Closure to update the top n PolyLines deque for the current contour.
            let mut ff = |pl: &PolyLine| {
                let d = calc_sco(pl);
                let len = top_in_cntr.len();
                if len > 0 {
                    if d < top_in_cntr.get(len - 1).unwrap().0 || len <= n {
                        if len == n {
                            top_in_cntr.pop_front();
                        }
                        top_in_cntr.push_back((d, pl.clone()));
                        top_in_cntr.make_contiguous().sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                    }
                } else {
                    top_in_cntr.push_back((d, pl.clone()));
                }
            };

            // Generate and rank PolyLines for the current contour using the `ff` closure.
            gen_lines.complete_line(&mut ff);

            // Add the ranked PolyLines for the current contour to the 2D Vector.
            // Use calc_hash() to map the PolyLine to a Vec<u8> for storage.
            top_heap.push(top_in_cntr.into_iter().map(|a| (a.0, a.1.calc_hash().to_vec())).collect());
        }
        top_heap
    }

    pub(crate) fn select_top_all_3(counters: &Vec<Vec<Vec2>>, depth: usize, grid_size: usize, rect: Rect) -> Vec<Vec<(f64, Vec<u8>)>> {
        let mut top_heap: Vec<Vec<(f64, Vec<u8>)>> = Vec::with_capacity(grid_size as usize);

        for cntr in counters.iter() {
            let mut top_in_cntr: Vec<(f64, PolyLine)> = Vec::with_capacity(depth);
            let cn = Cntr::new(Some(cntr.to_vec()), grid_size as i16, &rect);
            let zone = cn.line_zone();
            let mut gen_lines = GenPolyLines::new(zone, grid_size as i16);
            let start_point = Point2 { x: 0, y: 0 };

            gen_lines.line_buf.nodes.push(start_point);

            let cntr_size = cn.points.len();
            let calc_sco = |pl: &PolyLine|
                GenPolyLines::sco2(
                    &cn, &pl.line2points(cntr_size, &rect),
                );

            let mut ff = |pl: &PolyLine| {
                let d = calc_sco(pl);

                if let Some(_) = top_in_cntr.iter().find(|a| a.0 == d) {
                    return;
                } else {
                    if top_in_cntr.len() == depth {
                        let m = top_in_cntr.iter()
                            .enumerate()
                            .max_by(|(_, a), (_, b)|
                                a.0.partial_cmp(&b.0).unwrap_or(core::cmp::Ordering::Equal)
                            )
                            .map(|(index, _)| index);

                        if let Some(i) = m {
                            top_in_cntr[i] = (d, pl.clone());
                        }
                    } else {
                        top_in_cntr.push((d, pl.clone()));
                    }
                }
            };
            gen_lines.complete_line(&mut ff);
            top_heap.push(top_in_cntr.into_iter().map(|a| (a.0, a.1.calc_hash().to_vec())).collect());
        }
        top_heap
    }

    pub (crate) fn select_top_all_4(
        cntrs: &Vec<Vec<Vec2>>, depth: usize, grid_size: usize, rect: Rect,
    ) -> Vec<Vec<(f64, Vec<u8>)>> {

        let mut top_heap: Vec<Vec<(f64, Vec<u8>)>> = Vec::with_capacity(grid_size as usize);

        for cntr in cntrs.iter() {
            let mut top_in_cntr: Vec<(f64, PolyLine)> = Vec::with_capacity(depth);
            let cn = Cntr::new(Some(cntr.to_vec()), grid_size as i16, &rect);
            let zone = cn.line_zone();
            let mut gen_lines = GenPolyLines::new(zone, grid_size as i16);
            let start_point = Point2 { x: 0, y: 0 };

            gen_lines.line_buf.nodes.push(start_point);

            let cntr_size = cn.points.len();
            let calc_sco = |pl: &PolyLine|
                GenPolyLines::sco2(
                    &cn, &pl.line2points(cntr_size, &rect),
                );

            let mut ff = |pl: &PolyLine| {
                let d = calc_sco(pl);

                if let Some(_) = top_in_cntr.iter().find(|a| a.0 == d) {
                    return
                }
                else {
                    if top_in_cntr.len() == depth {
                        let m = top_in_cntr.iter()
                            .enumerate()
                            .max_by(|(_, a), (_, b)|
                                a.0.partial_cmp(&b.0).unwrap_or(core::cmp::Ordering::Equal)
                            );

                        if let Some((i, r)) = m {
                            if r.0 > d {
                                top_in_cntr[i] = (d, pl.clone());
                            }
                        }
                    } else {
                        top_in_cntr.push((d, pl.clone()));
                    }
                }
            };
            gen_lines.complete_line(&mut ff);
            top_heap.push(top_in_cntr.into_iter().map(|a| (a.0, a.1.calc_hash().to_vec())).collect());
        }
        top_heap
    }

    // This function recursively explores and completes a polyline
    // using the provided closure or function `F` to process each completed polyline.
    fn complete_line<F>(&mut self, f: &mut F) where F: FnMut(&PolyLine) {
        // Increment the recursion level counter
        self.lev += 1;

        // Store the last point in the polyline as the starting point
        let start_point = self.line_buf.nodes.last().unwrap().clone();
        // Store the first point in the polyline
        let first_point = self.line_buf.nodes.first().unwrap().clone();
        // Find neighboring nodes that can be connected to the starting point
        let neighbour_nodes = NeighbourNodes::new(&self.cells, &self.line_buf, start_point, self.line_buf.grid_size);

        // Iterate over neighbor nodes
        for p in neighbour_nodes.into_iter() {
            // If the current neighbor node is the first point, complete the polyline
            if p == first_point {
                self.line_buf.nodes.push(p);
                (*f)(&self.line_buf); // Process the completed polyline with the closure or function `F`
                self.line_buf.nodes.pop();
                continue;
            }

            // Add the current neighbor node to the polyline and then recursively call `complete_line` to explore further.
            self.line_buf.nodes.push(p);
            self.complete_line(f);
            self.line_buf.nodes.pop(); // Remove the added node after exploration is done
        }
        // Decrement the recursion level counter
        self.lev -= 1;
    }
}


#[allow(dead_code)]
#[derive(Clone)]
struct NeighbourNodes {
    pub(crate) neighbours: Vec<Point2<i32>>,
    grid_size: i16,
}

impl NeighbourNodes {
    fn new(permitted_points: &CellSet, line: &PolyLine, start_point: Point2<i32>, grid_size: i16) -> Self {
        Self {
            neighbours: Self::near_points(permitted_points, line, start_point, DISTANCE, grid_size),
            grid_size,
        }
    }

    fn near_points(z: &CellSet, line: &PolyLine, start_point: Point2<i32>, dist: i32, grid_size: i16) -> Vec<Point2<i32>> {
        let grid_size_i32 = grid_size as i32;
        let chk_zone = |i: i32, j: i32, z: &CellSet, line: &PolyLine| -> bool {
            if i < 0 || i >= grid_size_i32 || j < 0 || j >= grid_size_i32 {
                return false;
            }

            let first = line.nodes.first().unwrap().clone();
            if first == (Point2 { x: i, y: j }) && line.nodes.len() > 5 {
                return true;
            }
            if !z.contains(&(i, j)) {
                return false;
            }

            for Point2 { x: pi, y: pj } in line.nodes.iter() {
                if (pi - i).abs() < dist as i32 && (pj - j).abs() < dist as i32 {
                    return false;
                }
            }
            true
        };

        let Point2 { x: i0, y: j0 } = start_point;
        let mut v: Vec<Point2<i32>> = Vec::with_capacity(((grid_size - 1) * 4) as usize);

        let min_i = i0 - dist;
        let min_j = j0 - dist + 1;
        let max_i = i0 + dist;
        let max_j = j0 + dist - 1;

        for i in min_i..=max_i {
            let j = min_j - 1;
            if chk_zone(i, j, z, line) {
                v.push(Point2::new(i, j));
            }
        }

        for j in min_j..=max_j {
            let i = max_i;
            if chk_zone(i, j, z, line) {
                v.push(Point2::new(i, j));
            }
        }

        for i in min_i..=max_i {
            let j = max_j + 1;
            if chk_zone(i, j, z, line) {
                v.push(Point2::new(i, j));
            }
        }

        for j in min_j..=max_j {
            let i = min_i;
            if chk_zone(i, j, z, line) {
                v.push(Point2::new(i, j));
            }
        }

        v.clone()
    }
}

impl IntoIterator for NeighbourNodes {
    type Item = Point2<i32>;
    type IntoIter = NeighbourNodesIter;

    // note that into_iter() is consuming self
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            iter: self.neighbours.into_iter(),
        }
    }
}

struct NeighbourNodesIter {
    iter: IntoIter<Point2<i32>>,
}

impl<'a> Iterator for NeighbourNodesIter {
    type Item = Point2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}