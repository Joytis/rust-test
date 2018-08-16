#[allow(non_camel_case_types)]
pub struct git_revspec {

}

pub struct Fern {
	pub size: f64,
	pub growth_rate: f64
}

impl Fern {
	/// Simplate a fern growing for one day
	pub fn grow(&mut self) {
		self.size *= 1.0 + self.growth_rate;
	}
}

pub fn run_simulation(fern: &mut Fern, days: usize) {
	for _ in 0..days {
		fern.grow();
	}
}