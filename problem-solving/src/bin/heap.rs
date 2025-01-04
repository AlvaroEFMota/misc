use std::error::Error;

fn main() {}

struct MinHeap {
    heap: Vec<Option<i32>>,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            heap: vec![None; 10],
        }
    }

    fn insert(&mut self, value: i32, index: usize) -> Result<(), Box<dyn Error>> {
        if index >= self.heap.len() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Heap cheio",
            )));
        }
        if let Some(value) = &mut self.heap[index] {}
    }
}
