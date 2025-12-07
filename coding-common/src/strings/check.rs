pub fn is_blank(s: &str) -> bool {
    s.chars().all(|c| c.is_whitespace())
}


pub fn is_blank1(s: &str) -> bool {
    // 快速检查：如果是空字符串，直接返回 true
    if s.is_empty() {
        return true;
    }

    // 遍历所有字符，检查是否都是空白字符
    for c in s.chars() {
        if !c.is_whitespace() {
            return false;
        }
    }
    true
}