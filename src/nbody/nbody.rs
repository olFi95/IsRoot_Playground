use rand::Rng;
use wgpu::wgt::PollType;

pub trait NBody {
    async fn step(&mut self, steps: usize);
    async fn get_point_locations(&mut self) -> Vec<[f32; 2]>;
}

pub fn generate_test_data(n: usize) -> Vec<[f32; 2]> {
    let mut rng = rand::rng();
    (0..n)
        .map(|_| [rng.random_range(-1.0..=1.0), rng.random_range(-1.0..=1.0)])
        .collect()
}
