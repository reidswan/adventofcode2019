use std::iter;

pub trait GroupIterator<T, U> where U: iter::Iterator<Item=T> {
    fn group(self, n: usize)-> IterGroup<T, U>;
}

pub struct IterGroup<T, U> where U: iter::Iterator<Item=T> {
    size: usize,
    src: U
}

pub enum GroupedItem<T> {
    Complete(Vec<T>),
    Partial(Vec<T>)
}

impl<T, U> Iterator for IterGroup<T, U> where U: iter::Iterator<Item=T> {
    type Item = GroupedItem<T>;

    fn next(&mut self)-> Option<Self::Item> {
        let mut item: Vec<T> = Vec::with_capacity(self.size);
        
        let first = self.src.next()?;
        item.push(first);
        for _ in 1..self.size {
            let popped = self.src.next();
            item.push(match popped {
                None => return Some(GroupedItem::Partial(item)),
                Some(x) => x
            });
        }

        Some(GroupedItem::Complete(item))
    }
}

impl<T, U> GroupIterator<T, U> for U where U: Iterator<Item=T> {
    fn group(self, n: usize)-> IterGroup<T, U> {
        IterGroup {
            size: n,
            src: self
        }
    }
}