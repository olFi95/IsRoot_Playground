@group(0) @binding(0)
var<storage, read_write> point_location: array<vec2<f32>>;
@group(0) @binding(1)
var<storage, read> point_speed: array<vec2<f32>>;
@group(0) @binding(2)
var<storage, read> point_mass: array<f32>;
@group(0) @binding(3)
var<uniform> vector_length: u32;


@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) GlobalInvocationID : vec3<u32>) {
    var i = GlobalInvocationID.x;
    if (i >= vector_length) {
        return;
    }

    // N-Body simulation implementation
    var current_pos = point_location[i];
    var current_velocity = point_speed[i];
    var current_mass = point_mass[i]; // Assuming mass is stored in x component

    var force = vec2<f32>(0.0, 0.0);

    // Calculate gravitational force from all other bodies
    for (var j = 0u; j < vector_length; j++) {
        if (j == i) {
            continue; // Skip self-interaction
        }

        var other_pos = point_location[j];
        var other_mass = point_mass[j];

        // Calculate distance vector
        var r_vec = other_pos - current_pos;
        var r_squared = dot(r_vec, r_vec);

        // Avoid division by zero and singularities
        var epsilon = 1e-6;
        r_squared = max(r_squared, epsilon);

        // Calculate gravitational force magnitude: F = G * m1 * m2 / r^2
        // Using G = 1.0 for simplicity (can be adjusted as needed)
        var force_magnitude = current_mass * other_mass / r_squared;

        // Force direction (unit vector)
        var r_distance = sqrt(r_squared);
        var force_direction = r_vec / r_distance;

        // Add this force contribution
        force += force_magnitude * force_direction;
    }

    // Update velocity based on force (F = ma, so a = F/m)
    var acceleration = force / current_mass;
    var dt = 0.016; // Time step (assuming ~60 FPS, can be made uniform parameter)
    var new_velocity = current_velocity + acceleration * dt;

    // Update position based on velocity
    var new_position = current_pos + new_velocity * dt;

    // Store updated position
    point_location[i] = new_position;
}