// There's nothing to store values here you dolt

pub struct ListNode<T: Default> {
    pub val: T,
    pub size: usize,
    pub next: Option<&'static mut ListNode<T>>,
}

impl ListNode<T> {
    pub const fn new(size: usize) -> Self {
        ListNode { val: T::, size, next: None }
    }

    pub fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    pub fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}
