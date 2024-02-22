use std::simd::SimdPartialOrd;

pub struct VectorizedHeap {
    data: Vec<f32>,
    len: usize,
    keys: Vec<usize>,
    reverse: Vec<usize>,
}

const LANES: usize = 2 * SIMD_LANES;

const SIMD_LANES: usize = 1 << POWER;

const POWER: usize = 3;

type Vector<T> = std::simd::Simd<T, SIMD_LANES>;

impl VectorizedHeap {
    fn new_() -> VectorizedHeap {
        VectorizedHeap { data: Vec::new(), len: 0, keys: Vec::new(), reverse: Vec::new() }
    }

    fn push_(&mut self, elem: f32) {
        let index = self.len;
        self.keys.push(index);
        self.reverse.push(index);
        if index % LANES == 0 {
            self.data.extend_from_slice(&[f32::INFINITY; LANES]);
        }
        self.len += 1;
        self.data[index] = elem;
        self.bubble_up(index);
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index >= LANES {
            let parent = (index / LANES) - 1;
            if self.data[index] < self.data[parent] {
                self.swap(index, parent);
            }
            index = parent;
        }
    }

    fn bubble_down(&mut self, mut index: usize) {
        loop {
            let child = (index + 1) * LANES;
            if self.len <= child {
                break;
            }
            let child_min = self.find_min(child);
            if self.data[index] > self.data[child_min] {
                self.swap(index, child_min);
            }
            else {
                break;
            }
            index = child_min;
        }
    }

    fn pop_(&mut self) -> Option<(usize, f32)> {
        (self.len > 0).then(|| {
            let min = self.find_min(0);
            let val = self.data[min];
            let index = self.keys[min];
            //self.swap(min, self.len - 1);
            self.data[min] = self.data[self.len - 1];
            self.keys[min] = self.keys[self.len - 1];
            self.reverse[self.keys[min]] = min;
            self.len -= 1;
            self.data[self.len] = f32::INFINITY;
            self.bubble_down(min);
            (index, val)
        })
    }

    fn decrease_key_(&mut self, key: usize, new_val: f32) {
        let index = self.reverse[key];
        self.data[index] = new_val;
        self.bubble_up(index);
    }

    fn find_min(&self, i: usize) -> usize {
        i + min_index(&self.data[i..i + LANES])
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
        self.keys.swap(i, j);
        self.reverse.swap(self.keys[i], self.keys[j]);
    }
}

impl super::heap::Heap for VectorizedHeap {
    fn new() -> Self {
        Self::new_()
    }

    fn push(&mut self, elem: f32) {
        self.push_(elem)
    }

    fn pop(&mut self) -> Option<(usize, f32)> {
        self.pop_()
    }

    fn decrease_key(&mut self, index: usize, new_value: f32) {
        self.decrease_key_(index, new_value)
    }

    fn len(&self) -> usize {
        self.len
    }
}

const INDICES: [i32; LANES] = {
    let mut indices = [0; LANES];
    let mut i = 0;
    while i < LANES {
        indices[i] = i as i32;
        i += 1;
    }
    indices
};

fn f32_mask(simd: Vector<i32>) -> Vector<f32> {
    unsafe { std::mem::transmute(simd) }
}

fn i32_mask(simd: Vector<f32>) -> Vector<i32> {
    unsafe { std::mem::transmute(simd) }
}

fn f32_lt(a: Vector<i32>, b: Vector<i32>) -> Vector<i32> {
    f32_mask(a).simd_le(f32_mask(b)).to_int()
}

//#[inline(never)]
//#[no_mangle]
fn min_index(arr: &[f32]) -> usize {
    let mut a = i32_mask(Vector::<f32>::from_slice(&arr[..SIMD_LANES]));
    let mut a_indices = Vector::<i32>::from_slice(&INDICES[..SIMD_LANES]);
    let mut b = i32_mask(Vector::<f32>::from_slice(&arr[SIMD_LANES..]));
    let mut b_indices = Vector::<i32>::from_slice(&INDICES[SIMD_LANES..]);
    let lt = f32_lt(a, b);
    let mut indicies = (a_indices & lt) | (b_indices & !lt);
    let mut elements = (a & lt) | (b & !lt);
    for _ in 0..POWER {
        (a_indices, b_indices) = indicies.deinterleave(indicies);
        (a, b) = elements.deinterleave(elements);
        let lt = f32_lt(a, b);
        indicies = (a_indices & lt) | (b_indices & !lt);
        elements = (a & lt) | (b & !lt);
    }
    indicies[0] as usize
}
