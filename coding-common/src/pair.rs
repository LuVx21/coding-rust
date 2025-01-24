#[derive(Debug, Clone)]
pub struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    // 构造函数
    pub fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn second(&self) -> &U {
        &self.second
    }

    // 获取第一个值的可变引用
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.first
    }

    // 获取第二个值的可变引用
    pub fn second_mut(&mut self) -> &mut U {
        &mut self.second
    }

    // 交换两个值
    pub fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

impl<T: PartialEq, U: PartialEq> PartialEq for Pair<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}


impl<T, U> From<Pair<T, U>> for (T, U) {
    fn from(pair: Pair<T, U>) -> Self {
        (pair.first, pair.second)
    }
}

impl<T: Default, U: Default> Default for Pair<T, U> {
    fn default() -> Self {
        Pair {
            first: T::default(),
            second: U::default(),
        }
    }
}
