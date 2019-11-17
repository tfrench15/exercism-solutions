
#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        CustomSet {
            data: input.to_vec(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(&element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        unimplemented!();
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!();
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        unimplemented!();
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    pub fn difference(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    pub fn union(&self, _other: &Self) -> Self {
        unimplemented!();
    }
}
