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
    for (input, expected) in &find_peak_element::get_test_cases() {
        let mut result = find_peak_element::solve(input.to_owned());

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
