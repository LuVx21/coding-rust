use coding_common::pair::Pair;

#[test]
fn test_pair() {
    let mut pair = Pair::new(42, "hello");
    println!("Pair: {:?}", pair);
    println!("{} = {}", pair.first(), pair.second());

    *pair.first_mut() = 100;
    *pair.second_mut() = "world";

    println!("修改后: {:?}", pair);
    println!("交换后: {:?}", pair.swap());

    println!("{}", "----------------");

    let pair1 = Pair::new(42, "hello");
    let pair2 = pair1.clone();
    println!("是否相等: {}", pair1 == pair2);
    println!("{}", "----------------");

    let pair = Pair::new(42, "hello");
    let tuple: (i32, &str) = pair.into();
    println!("Tuple: {:?}", tuple);
    println!("{}", "----------------");

    let pair: Pair<i32, String> = Pair::default();
    println!("默认: {:?}", pair);
}
