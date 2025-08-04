pub trait Root {
    fn is_root(
        squareroot: &Vec<f32>,
        input: &Vec<f32>,
        delta: f32,
    ) -> impl Future<Output = Option<bool>> + Send;
}
