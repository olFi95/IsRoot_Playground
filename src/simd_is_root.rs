use crate::is_root::Root;
use std::simd::num::SimdFloat;
use std::simd::{LaneCount, Simd, SupportedLaneCount};

pub struct SimdIsRoot<const N: usize>;
impl<const N: usize> Root for SimdIsRoot<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    async fn is_root(squareroot: &Vec<f32>, input: &Vec<f32>, delta: f32) -> Option<bool> {
        if squareroot.len() != input.len() {
            return None;
        }
        for (squareroot_chunk, input_chunk) in squareroot.chunks_exact(N).zip(input.chunks_exact(N))
        {
            let squareroot_chunk_simd = Simd::<f32, N>::from_slice(squareroot_chunk);
            let input_chunk_simd = Simd::<f32, N>::from_slice(input_chunk);
            if ((squareroot_chunk_simd * squareroot_chunk_simd) - input_chunk_simd).abs()
                > Simd::<f32, N>::splat(delta)
            {
                return Some(false);
            }
        }
        for (squareroot_chunk, input_chunk) in squareroot
            .chunks_exact(N)
            .remainder()
            .into_iter()
            .zip(input.chunks_exact(N).remainder())
        {
            if (squareroot_chunk * squareroot_chunk - input_chunk).abs() > delta {
                return Some(false);
            }
        }
        Some(true)
    }
}

#[cfg(test)]
mod test {
    use crate::is_root::Root;
    use crate::simd_is_root::SimdIsRoot;

    #[tokio::test]
    async fn test_is_root_simd() {
        let squareroot = &vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let input = &vec![1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0];
        let result = SimdIsRoot::<4>::is_root(squareroot, input, 0.001).await;
        assert!(result.is_some());
        assert_eq!(result.unwrap(), true);
    }
    #[tokio::test]
    async fn test_is_root_simd_with_remainder() {
        let squareroot = &vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let input = &vec![1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0];
        let result = SimdIsRoot::<2>::is_root(squareroot, input, 0.001).await;
        assert!(result.is_some());
        assert_eq!(result.unwrap(), true);
    }
}
