type Pair = (Box<[u8]>, Box<[u8]>);

pub fn pair(left: &[u8], right: &[u8]) -> Pair {
    (Box::from(left), Box::from(right))
}
