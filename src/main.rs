use heaps::{heap::Heap, vectorized_heap::VectorizedHeap, binary_heap::BinaryHeap};

fn main() {
    //bench::<VectorizedHeap>();
    //bench::<BinaryHeap>();
    let data = gen_dijkstra(10_000_000, 10);
    let vect = dijkstra::<VectorizedHeap>(&data);
    let vect2 = dijkstra_(&data);
    let bina = dijkstra::<BinaryHeap>(&data);
    assert_eq!(vect, vect2);
}

const SIZE: usize = 100_000_000;

fn bench<H: Heap>() {
    let mut rand = Vec::with_capacity(SIZE);
    let mut res = Vec::with_capacity(SIZE);
    for _ in 0..SIZE {
        rand.push(fastrand::f32());
    }
    let now = std::time::Instant::now();
    let mut heap = H::new();
    for &k in &rand {
        heap.push(k);
    }
    for _ in 0..SIZE {
        res.push(heap.pop().unwrap());
    }
    dbg!(now.elapsed());
}

fn gen_dijkstra(nodes: usize, edgez: usize) -> Vec<Vec<(usize, f32)>> {
    let mut edges = vec![vec![]; nodes];
    for i in 0..nodes {
        for _ in 0..edgez {
            edges[i].push((fastrand::usize(..nodes), fastrand::f32()));
        }
    }
    edges
}

fn dijkstra<H: Heap>(edges: &Vec<Vec<(usize, f32)>>) -> Vec<f32> {
    let mut res = vec![f32::INFINITY; edges.len()];
    res[0] = 0.0;
    let now = std::time::Instant::now();
    let mut heap = H::new();
    heap.push(0.0);
    for _ in 1..edges.len() {
        heap.push(f32::INFINITY);
    }
    while let Some((index, val)) = heap.pop() {
        for &(to, w) in &edges[index] {
            if val + w < res[to] {
                res[to] = val + w;
                heap.decrease_key(to, val + w);
            }
        }
    }
    dbg!(now.elapsed());
    res
}

fn dijkstra_(edges: &Vec<Vec<(usize, f32)>>) -> Vec<f32> {
    let mut res = vec![f32::INFINITY; edges.len()];
    res[0] = 0.0;
    let now = std::time::Instant::now();
    let mut heap = heaps::vectorized_heap2::VectorizedHeap::new(edges.len());
    heap.push(0.0, 0);
    while let Some((index, val)) = heap.pop() {
        for &(to, w) in &edges[index] {
            if val + w < res[to] {
                res[to] = val + w;
                heap.push_or_decrease_key(to, val + w);
            }
        }
    }
    dbg!(now.elapsed());
    res
}
