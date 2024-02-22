use std::simd::{cmp::SimdPartialOrd, num::{SimdFloat, SimdInt}, simd_swizzle, Simd};

pub struct VectorizedHeap {
    data: Vec<Group>,
    len: usize,
    reverse: Vec<usize>,
}

struct Group {
    keys: [usize; LANES],
    data: [f32; LANES],
}

impl Group {
    fn new(index: usize) -> Group {
        Group { keys: [0; LANES], data: [f32::INFINITY; LANES] }
    }
}

const LANES: usize = 2 * SIMD_LANES;

const SIMD_LANES: usize = 1 << POWER;

const POWER: usize = 3;

impl VectorizedHeap {
    pub fn new(size: usize) -> VectorizedHeap {
        VectorizedHeap { data: Vec::new(), len: 0, reverse: vec![usize::MAX; size] }
    }

    pub fn push(&mut self, elem: f32, key: usize) {
        let index = self.len;
        if index % LANES == 0 {
            self.data.push(Group::new(index));
        }
        unsafe {
            *self.raw_key(index) = key;
            *self.raw_data(index) = elem;
        }
        self.reverse[key] = index;
        self.len += 1;
        self.bubble_up(index);
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index >= LANES {
            let parent = (index / LANES) - 1;
            if self.data(index) < self.data(parent) {
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
            let child_min = self.find_min(child / LANES) + child;
            if self.data(index) > self.data(child_min) {
                self.swap(index, child_min);
            }
            else {
                break;
            }
            index = child_min;
        }
    }

    pub fn pop(&mut self) -> Option<(usize, f32)> {
        (self.len > 0).then(|| {
            let min = self.find_min(0);
            let val = self.data[0].data[min];
            let index = self.data[0].keys[min];
            //self.swap(min, self.len - 1);
            self.data[0].data[min] = self.data(self.len - 1);
            self.data[0].keys[min] = self.key(self.len - 1);
            let v = self.key(min);
            self.reverse[v] = min;
            self.len -= 1;
            unsafe {
                *self.raw_data(self.len) = f32::INFINITY;
            }
            self.bubble_down(min);
            self.reverse[index] = usize::MAX;
            (index, val)
        })
    }

    pub fn decrease_key(&mut self, key: usize, new_val: f32) {
        let index = self.reverse[key];
        unsafe {
            *self.raw_data(index) = new_val;
        }
        self.bubble_up(index);
    }

    pub fn push_or_decrease_key(&mut self, key: usize, new_val: f32) {
        if self.reverse[key] == usize::MAX { 
            self.push(new_val, key);
        } 
        else { 
            self.decrease_key(key, new_val); 
        }
    }

    fn find_min(&self, i: usize) -> usize {
        min_index(self.data[i].data)
    }

    fn key(&self, i: usize) -> usize {
        self.data[i / LANES].keys[i % LANES]
    }
    
    fn data(&self, i: usize) -> f32 {
        self.data[i / LANES].data[i % LANES]
    }

    unsafe fn raw_data(&mut self, i: usize) -> &'static mut f32 {
        std::mem::transmute(&mut self.data[i / LANES].data[i % LANES])
    }
    
    unsafe fn raw_key(&mut self, i: usize) -> &'static mut usize {
        std::mem::transmute(&mut self.data[i / LANES].keys[i % LANES])
    }

    fn swap(&mut self, i: usize, j: usize) {
        unsafe {
            std::mem::swap(self.raw_data(i), self.raw_data(j));
            std::mem::swap(self.raw_key(i), self.raw_key(j));
        }
        let a = self.key(i);
        let b = self.key(j);
        self.reverse.swap(a, b);
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

const INDICES: [u32; LANES] = {
    let mut indices = [0; LANES];
    let mut i = 0;
    while i < LANES {
        indices[i] = i as u32;
        i += 1;
    }
    indices
};

fn min_index(arr: [f32; LANES]) -> usize {
    let a_values: Simd<f32, 8> = Simd::from_slice(&arr[..8]);
    let a_indices: Simd<u32, 8> = Simd::from_slice(&INDICES[..8]);
    let b_values = Simd::from_slice(&arr[8..]);
    let b_indices = Simd::from_slice(&INDICES[8..]);
    let mask = a_values.simd_le(b_values).to_int().cast();
    let values = Simd::from_bits(a_values.to_bits() & mask | b_values.to_bits() & !mask);
    let indices = a_indices & mask | b_indices & !mask;
    let a_values: Simd<f32, 4> = simd_swizzle!(values, [0, 1, 2, 3]);
    let b_values: Simd<f32, 4> = simd_swizzle!(values, [4, 5, 6, 7]);
    let a_indices = simd_swizzle!(indices, [0, 1, 2, 3]);
    let b_indices = simd_swizzle!(indices, [4, 5, 6, 7]);
    let mask = a_values.simd_le(b_values).to_int().cast();
    let values = Simd::from_bits(a_values.to_bits() & mask | b_values.to_bits() & !mask);
    let indices = a_indices & mask | b_indices & !mask;
    let a_values: Simd<f32, 2> = simd_swizzle!(values, [0, 1]);
    let b_values: Simd<f32, 2> = simd_swizzle!(values, [2, 3]);
    let a_indices = simd_swizzle!(indices, [0, 1]);
    let b_indices = simd_swizzle!(indices, [2, 3]);
    let mask = a_values.simd_le(b_values).to_int().cast();
    let values = Simd::from_bits(a_values.to_bits() & mask | b_values.to_bits() & !mask);
    let indices = a_indices & mask | b_indices & !mask;
    let a_values: Simd<f32, 1> = simd_swizzle!(values, [0]);
    let b_values: Simd<f32, 1> = simd_swizzle!(values, [1]);
    let a_indices = simd_swizzle!(indices, [0]);
    let b_indices = simd_swizzle!(indices, [1]);
    let mask = a_values.simd_le(b_values).to_int().cast();
    let indices = a_indices & mask | b_indices & !mask;
    indices[0] as usize
}
