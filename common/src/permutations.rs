pub use streaming_iterator::StreamingIterator;
use std::mem;

pub struct Permutations<T> {
    is_first: bool,
    perms: Option<HeapPermutation<T>>
}

impl<T> StreamingIterator for Permutations<T> where T: Clone {
    type Item = Vec<T>;

    fn advance(&mut self) {
        if self.is_first {
            self.is_first = false
        } else {
            self.perms = self.perms.take().and_then(|a| a.shuffle());
        }
    }

    fn get<'a>(&'a self)-> Option<&'a Self::Item> {
        if let Some(p) = &self.perms {
            Some(&p.src)
        } else {
            None
        }
    }
}

pub trait Permutable<T> {
    fn permutations(&self) -> Permutations<T>;
}

impl<T> Permutable<T> for Vec<T>
where
    T: Clone,
{
    fn permutations(&self) -> Permutations<T> {
        Permutations {
            is_first: true,
            perms: Some(HeapPermutation::new(&self))
        }
    }
}

#[derive(Debug)]
struct HeapPermutation<T> {
    src: Vec<T>,
    stack: Vec<usize>,
    stack_pointer: usize,
}

impl<T> HeapPermutation<T>
where
    T: Clone,
{
    fn new(src: &Vec<T>) -> Self {
        HeapPermutation {
            src: src.clone(),
            stack: vec![0usize; src.len()],
            stack_pointer: 0,
        }
    }

    fn shuffle(mut self) -> Option<Self> {
        if self.stack_pointer >= self.stack.len() {
            return None;
        }

        if self.stack[self.stack_pointer] < self.stack_pointer {
            if self.stack_pointer % 2 == 0 {
                swap(&mut self.src, 0, self.stack_pointer);
            } else {
                swap(
                    &mut self.src,
                    self.stack_pointer,
                    self.stack[self.stack_pointer],
                );
            }
            self.stack[self.stack_pointer] += 1;
            self.stack_pointer = 0;
            Some(self)
        } else {
            self.stack[self.stack_pointer] = 0;
            self.stack_pointer += 1;
            self.shuffle()
        }
    }
}

fn swap<T>(src: &mut [T], index1: usize, index2: usize) {
    if index1 == index2 {
        return;
    }

    let (smaller, larger) = if index1 < index2 {
        (index1, index2)
    } else {
        (index2, index1)
    };
    let (a, b) = src.split_at_mut(smaller + 1);
    mem::swap(&mut a[smaller], &mut b[larger - smaller - 1])
}
