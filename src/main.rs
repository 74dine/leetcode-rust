mod design_hashset;
mod group_anagrams;
mod longest_common_prefix;
mod longest_consecutive_sequence;
mod majority_element;
mod max_consecutive_ones;
mod remove_element;
mod reverse_string;
mod set_mismatch;
mod shuffle_the_array;
mod sort_an_array;
mod sort_colors;
mod template;
mod top_k_frequent_elements;
mod two_sum;
mod valid_palindrome;
mod valid_palindrome_ii;
mod merge_alternately;

// #[allow(unreachable_code)]
#[allow(dead_code)]
fn main() {
    // println!("{}", 'A' as u8);
    // println!("{}", 122 as char);
    // for code in 0..=97 {
    //     println!("[{code}] {}", code as u8 as char);
    // }

    run_tests();
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn run_tests() {
    for (input, expected) in &merge_alternately::get_test_cases() {
        let mut result = merge_alternately::solve((*input).0.to_owned(), (*input).1.to_owned());

        // result = sort_colors::organize_result().unwrap();

        if result == *expected {
            println!("[passed] Case {:?} => {:?}", input, expected);
        } else {
            println!(
                "[failed] Case {:?} => {:?} | Returned: {:?}",
                input, expected, result
            );
        }
    }
}
