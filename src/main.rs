mod design_hashset;
mod group_anagrams;
mod longest_common_prefix;
mod longest_consecutive_sequence;
mod majority_element;
mod max_consecutive_ones;
mod merge_sorted_array;
mod merge_strings_alternately;
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
mod baseball_game;
mod find_peak_element;
mod valid_parentheses;

// #[allow(unreachable_code)]
#[allow(dead_code)]
fn main() {
    // println!("{}", '(' as u8);
    // println!("{}", '[' as u8);
    // println!("{}", '{' as u8);
    // return;

    run_tests();
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn run_tests() {
    for (input, expected) in &valid_parentheses::get_test_cases() {
        let mut result = valid_parentheses::solve(input.to_owned());

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
