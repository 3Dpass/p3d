#[derive(Debug, Clone, Copy)]
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
	pub(crate) fn min_x(&self) -> f64 { self.min_x }
	pub(crate) fn min_y(&self) -> f64 { self.min_y }
}