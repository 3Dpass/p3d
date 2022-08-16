use alloc::vec::Vec;
use alloc::collections::BTreeSet as HashSet;

use cgmath::Point2;

#[allow(unused_imports)]
use cgmath::num_traits::real::Real; // round

pub(crate) type CellSet = HashSet<(i32, i32)>;

#[derive(Debug, Clone)]
pub(crate) struct Rect {
	min_x: f64, max_x: f64,
	min_y: f64, max_y: f64,
}

impl Rect {
	pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
		Self {
			min_x, max_x, min_y, max_y,
		}
	}
	pub(crate) fn width(&self) -> f64 { self.max_x - self.min_x }
	pub(crate) fn height(&self) -> f64 { self.max_y - self.min_y }
}

pub(crate) struct Cntr {
	pub(crate) points: Vec<Point2<f64>>,
	pub(crate) n_size: i16,
	pub(crate) rect: Rect,
}

impl Cntr{
	pub(crate) fn new(
		vp: Option<Vec<Point2<f64>>>,
		grid_size: i16,
		rect: &Rect,
	) -> Self {
		Self {
			points: match vp {
				Some(v) => v,
				None => vec![],
			},
			n_size: grid_size,
			rect: rect.clone(),
		}
	}

	pub(crate) fn push(&mut self, p: Point2<f64>) {
		self.points.push(p);
	}

	pub(crate) fn line_zone(&self) -> CellSet {
		fn chk_add(z: &mut CellSet, n: i32, i: i32, j: i32) {
			if (0..n).contains(&i) && (0..n).contains(&j) {
				z.insert((i, j));
			}
		}

		let n = self.n_size as i32;
		// let min_x = self.points.iter().map(|a| a.x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
		// let min_y = self.points.iter().map(|a| a.y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
		// let max_x = self.points.iter().map(|a| a.x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
		// let max_y = self.points.iter().map(|a| a.y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

		let w = self.rect.width();
		let h = self.rect.height();

		let grid_dx: f64 = w / n as f64;
		let grid_dy: f64 = h / n as f64;

		let dx = 0.1 * grid_dx;
		let dy = 0.1 * grid_dy;

		let _len = self.points.len();

		let mut z = CellSet::new();

		for pp in self.points.iter() {
			let Point2{x: px, y: py} = *pp;
			let mut i = ((px - self.rect.min_x) / grid_dx) as i32;
			let mut j = ((py - self.rect.min_y) / grid_dy) as i32;

			if i == n {
				i = n - 1;
			}
			if j == n {
				j = n - 1;
			}

			z.insert((i, j));

			let xx = px.round() - px;
			let yy = py.round() - py;

			if xx.abs() < dx {
				if xx >= 0.0 {
					chk_add(&mut z, n, i + 1, j);
				} else {
					chk_add(&mut z, n, i - 1, j);
				}
			}
			if yy.abs() < dy {
				if yy >= 0.0 {
					chk_add(&mut z, n, i, j + 1);
				} else {
					chk_add(&mut z, n, i, j - 1);
				}
			}

			if yy.abs() < dy && xx.abs() < dx {
				if xx >= 0.0 && yy >= 0.0 {
					chk_add(&mut z, n, i + 1, j + 1);
				}
				if xx >= 0.0 && yy < 0.0 {
					chk_add(&mut z, n, i + 1, j - 1);
				}
				if xx < 0.0 && yy >= 0.0 {
					chk_add(&mut z, n, i - 1, j + 1);
				}
				if xx < 0.0 && yy < 0.0 {
					chk_add(&mut z, n, i - 1, j - 1);
				}
			}
		}
		// a = set()
		// for i in range(n):
		//   for j in range(n):
		//     a.add((i, j))
		//
		// notz = a - z
		z.clone()
	}


}
