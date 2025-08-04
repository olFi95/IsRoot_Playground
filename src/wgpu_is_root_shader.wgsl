@group(0) @binding(0)
var<storage, read> squareroots: array<f32>;

@group(0) @binding(1)
var<storage, read> inputs: array<f32>;

@group(0) @binding(2)
var<uniform> delta: f32;

@group(0) @binding(3)
var<storage, read_write> result: atomic<u32>;

@group(0) @binding(4)
var<uniform> vector_length: u32;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) GlobalInvocationID : vec3<u32>) {
    var i = GlobalInvocationID.x;
    if (i >= vector_length) {
        return;
    }
    let diff = abs((squareroots[i] * squareroots[i]) - inputs[i]);
    if (diff > delta) {
        // Setze Ergebnis auf 0 (false), falls eine Abweichung zu groß ist
        atomicStore(&result, 0u);
    }
}