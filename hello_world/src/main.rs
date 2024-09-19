// fn most_frequent_word(text: &str) -> (String, usize) {
//     let words:Vec<&str> = text.split_whitespace().collect();
//     let mut max_word:&str = "";
//     let mut max_cnt:usize = 0;


//     for idx:usize in 0..words.len() {
//         let word:&str = words[idx];
//         let mut cnt:usize = 0;

//         for idy:usize in 0..words.len() {
//             if words[idy] == word {
//                 cnt += 1;
//             }
//             if cnt > max_cnt{
//                 max_cnt = cnt;
//                 max_word = word;
//             }
//         }

//     }

//     return (max_word, max_cnt)
    

// }

// fn main() {
//     let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
//     let (word, count) = most_frequent_word(text);
//     println!("Most frequent word: \"{}\" ({} times)", word, count);
// }

// fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
//     // let mut low:i32 = low;

//     // while low <= high{
//     //     *total += low; 
//     //     low += step;
//     // }

//     for num:i32 in (low..high).step_by(step as usize){
//         *total += num; 
//     }

// }

// fn main() {
//     let mut result = 0;
//     sum_with_step(&mut result, 0, 100, 1);
//     println!("Sum 0 to 100, step 1: {}", result);

//     result = 0;
//     sum_with_step(&mut result, 0, 10, 2);
//     println!("Sum 0 to 10, step 2: {}", result);

//     result = 0;
//     sum_with_step(&mut result, 5, 15, 3);
//     println!("Sum 5 to 15, step 3: {}", result);
// }

// fn concat_strings(s1: &String, s2: &String) -> String {
//     let mut new_word: String = (*s1).clone();
//     new_word.push_str(string: s2);
//     new_word
// }

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("World!");
//     let result = concat_strings(&s1, &s2);
//     println!("{}", result); // Should print: "Hello, World!"
// }