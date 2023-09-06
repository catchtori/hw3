use std::ops::Deref;

struct MyRc<T> {
    data: T,
    ref_count: usize,
}

impl<T> MyRc<T> {
    fn new(data: T) -> Self {
        MyRc {
            data,
            ref_count: 1,
        }
    }

    fn clone(&self) -> Self {
        MyRc {
            data: self.data.clone(),
            ref_count: self.ref_count + 1,
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        if self.ref_count == 1 {
            println!("Dropping data: {:?}", self.data);
        } else {
            self.ref_count -= 1;
        }
    }
}

fn main() {
    let data1 = MyRc::new("Hello, World!".to_string());
    let data2 = data1.clone();

    println!("data1: {}", *data1);
    println!("data2: {}", *data2);
}
