pub struct Queue<T> {
    queue: Vec<T>,
    items_in_q: usize
}

impl<T> Queue<T> {
    pub fn new(n: usize) -> Self {

        Queue { 
            queue: Vec::with_capacity(n),
            items_in_q: 0    
        }
    }

    pub fn push(&mut self, x: T) {
        self.items_in_q += 1;
        self.queue.push(x);
    }

    pub fn pop(&mut self) -> T {
        self.items_in_q -= 1;
        self.queue.remove(0)
    }
    
    pub fn get_items_in_q(&self) -> usize {
        self.items_in_q
    }
    
    pub fn get_length(&self) -> usize {
        self.queue.len()
    }
    
    pub fn get_at(&self, i: usize) -> Option<&T> {
        if i < self.items_in_q {
            self.queue.get(i)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl<T: Clone> Queue<T> {
    pub fn fill_with(&mut self, value: T) {
        self.queue = vec![value; self.queue.capacity()];
    }
}