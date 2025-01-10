#[derive(Debug)]
struct MinHeap {
    vec: Vec<i32>,
    size: usize,
}

impl MinHeap {
    fn new(cap: usize) -> Self {
        MinHeap {
            vec: vec![-1; cap],
            size: 0,
        }
    }

    fn insert(&mut self, value: i32) {
        self.vec[self.size] = value;
        self.heapfy(self.size);
        self.size += 1;
    }

    fn heapfy(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.vec[parent] > self.vec[index] {
                let tmp = self.vec[parent];
                self.vec[parent] = self.vec[index];
                self.vec[index] = tmp;
            } else {
                break;
            }
            index = parent;
        }
    }

    fn pop(&mut self) -> i32 {
        if self.size > 0 {
            let top = self.vec[0];
            self.size -= 1;
            self.vec[0] = self.vec[self.size];
            self.vec[self.size] = -1;
            self.bubble_down();
            top
        } else {
            panic!("Empty heap");
        }
    }

    fn bubble_down(&mut self) {
        let mut index = 0;
        while index < self.size {
            let (left_child, right_child) = (2 * index + 1, 2 * index + 2);
            let child = if left_child < self.size && right_child < self.size {
                if self.vec[left_child] <= self.vec[right_child] {
                    left_child
                } else {
                    right_child
                }
            } else {
                if left_child < self.size {
                    left_child
                } else if right_child < self.size {
                    right_child
                } else {
                    index
                }
            };

            if self.vec[index] > self.vec[child] {
                let tmp = self.vec[index];
                self.vec[index] = self.vec[child];
                self.vec[child] = tmp;
            } else {
                break;
            }

            index = child;
        }
    }
}

fn main() {
    let mut min_heap = MinHeap::new(10);
    min_heap.insert(3);
    min_heap.insert(4);
    min_heap.insert(7);
    min_heap.insert(2);
    min_heap.insert(8);
    min_heap.insert(1);
    min_heap.insert(9);
    min_heap.insert(6);
    min_heap.insert(6);

    println!("{:?}", min_heap);
    println!("{:?}", min_heap.pop());
    println!("{:?}", min_heap);
    println!("{:?}", min_heap.pop());
    println!("{:?}", min_heap);
    println!("{:?}", min_heap.pop());
    println!("{:?}", min_heap);
    println!("{:?}", min_heap.pop());
    println!("{:?}", min_heap);
    println!("{:?}", min_heap.pop());
    println!("{:?}", min_heap);
    println!("{:?}", min_heap.pop());
    println!("{:?}", min_heap);
    println!("{:?}", min_heap.pop());
    println!("{:?}", min_heap);
    println!("{:?}", min_heap.pop());
    println!("{:?}", min_heap);
    println!("{:?}", min_heap.pop());
    println!("{:?}", min_heap);
}
