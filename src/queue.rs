#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
pub struct Queue<T> {
    num: i32,
    content: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            num: 0,
            content: Vec::new(),
        }
    }

    pub fn put(&mut self, item: T) {
        self.num += 1;
        self.content.push(item);
    }

    pub fn get(&mut self) -> T {
        self.num -= 1;
        self.content.pop().unwrap()
    }

    pub fn len(&self) -> i32 {
        self.num
    }
}

#[test]
fn test_get_put() {
    let mut q: Queue<String> = Queue::new();
    q.put("xx".to_string());
    assert_eq!(1, q.len());
    assert_eq!("xx".to_string(), q.get());
}
