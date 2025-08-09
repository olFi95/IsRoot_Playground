use crate::nbody::nbody::NBody;

pub struct NBodyCPU {
    point_location: Vec<[f32; 2]>,
    point_mass: Vec<f32>,
    point_velocity: Vec<[f32; 2]>,
}

impl NBodyCPU {
    pub fn new(start_locations: &Vec<[f32; 2]>) -> Self {
        NBodyCPU{
            point_location: start_locations.clone(),
            point_mass: vec![100.0f32; start_locations.len()],
            point_velocity: vec![[0.0f32; 2]; start_locations.len()],
        }
    }
}

impl NBody for NBodyCPU{
    async fn step(&mut self, steps: usize) {
        let dt = 0.016f32;
        let epsilon = 1e-6;
        let n = self.point_location.len();

        for _ in 0..steps {
            let mut new_positions = vec![[0.0f32; 2]; n];
            let mut new_velocities = vec![[0.0f32; 2]; n];

            for i in 0..n {
                let current_pos = self.point_location[i];
                let current_velocity = self.point_velocity[i];
                let current_mass = self.point_mass[i];

                let mut force = [0.0f32, 0.0f32];

                for j in 0..n {
                    if i == j {
                        continue;
                    }

                    let other_pos = self.point_location[j];
                    let other_mass = self.point_mass[j];

                    let r_vec = [
                        other_pos[0] - current_pos[0],
                        other_pos[1] - current_pos[1],
                    ];

                    let r_squared = (r_vec[0].powi(2) + r_vec[1].powi(2)).max(epsilon);
                    let force_magnitude = (current_mass * other_mass) / r_squared;
                    let r_distance = r_squared.sqrt();

                    let force_direction = [
                        r_vec[0] / r_distance,
                        r_vec[1] / r_distance,
                    ];

                    force[0] += force_magnitude * force_direction[0];
                    force[1] += force_magnitude * force_direction[1];
                }

                // F = ma -> a = F / m
                let acceleration = [
                    force[0] / current_mass,
                    force[1] / current_mass,
                ];

                // v = v + a * dt
                let new_velocity = [
                    current_velocity[0] + acceleration[0] * dt,
                    current_velocity[1] + acceleration[1] * dt,
                ];

                // x = x + v * dt
                let new_position = [
                    current_pos[0] + new_velocity[0] * dt,
                    current_pos[1] + new_velocity[1] * dt,
                ];

                new_positions[i] = new_position;
                new_velocities[i] = new_velocity;
            }

            // Apply updated positions and velocities
            self.point_location = new_positions;
            self.point_velocity = new_velocities;
        }
    }

    async fn get_point_locations(&mut self) -> Vec<[f32; 2]> {
        self.point_location.clone()
    }
}
