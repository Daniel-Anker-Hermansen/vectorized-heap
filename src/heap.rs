pub trait Heap {
    fn new() -> Self;

    fn push(&mut self, elem: f32);

    fn pop(&mut self) -> Option<(usize, f32)>;

    fn decrease_key(&mut self, index: usize, new_value: f32);

    fn len(&self) -> usize;
}
