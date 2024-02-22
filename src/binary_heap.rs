pub struct BinaryHeap {
    data: Vec<f32>,
    keys: Vec<usize>,
    reverse: Vec<usize>,
}

impl BinaryHeap {
    fn new_() -> BinaryHeap {
        BinaryHeap { data: Vec::new(), keys: Vec::new(), reverse: Vec::new() }
    }

    fn push_(&mut self, elem: f32) {
        let index = self.data.len();
        self.keys.push(index);
        self.reverse.push(index);
        self.data.push(elem);
        self.bubble_up(index);
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.data[index] < self.data[parent] {
                self.swap(index, parent);
            }
            index = parent;
        }
    }

    fn bubble_down(&mut self, mut index: usize) {
        loop {
            let left_child = index * 2 + 1;
            let right_child = left_child + 1;
            if self.data.len() <= left_child {
                break;
            }
            let child = if self.data.len() > right_child && self.data[right_child] < self.data[left_child] { right_child } else { left_child };
            if self.data[child] < self.data[index] {
                self.swap(index, child);
            }
            else {
                break;
            }
            index = child;
        }
    }

    fn pop_(&mut self) -> Option<(usize, f32)> {
        (self.data.len() > 0).then(|| {
            let val = self.data[0];
            let index = self.keys[0];
            //self.swap(0, self.len() - 1);
            self.data[0] = self.data[self.data.len() - 1];
            self.keys[0] = self.keys[self.data.len() - 1];
            self.reverse[self.keys[0]] = 0;
            unsafe { self.data.set_len(self.data.len() - 1) };
            self.bubble_down(0);
            (index, val)
        })
    }

    fn decrease_key_(&mut self, key: usize, new_val: f32) {
        let index = self.reverse[key];
        self.data[index] = new_val;
        self.bubble_up(index);
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
        self.keys.swap(i, j);
        self.reverse.swap(self.keys[i], self.keys[j]);
    }
}

impl super::heap::Heap for BinaryHeap {
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
        self.data.len()
    }
}
