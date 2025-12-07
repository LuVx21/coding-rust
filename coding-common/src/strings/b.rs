pub fn first_non_empty<'a>(strings: &[&'a str]) -> Option<&'a str> {
    strings.iter().find(|&&s| !s.is_empty()).copied()
}

// macro_rules! first_non_empty_macro {
//     ($($arg:expr),+ $(,)?) => {{
//         let result = None;
//         $(
//             let result = if result.is_some() {
//                 result
//             } else if !$arg.is_empty() {
//                 Some($arg)
//             } else {
//                 None
//             };
//         )+
//         result
//     }};
// }