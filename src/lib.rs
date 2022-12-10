
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

mod invert_values;
mod find_multiples_of_number;
mod subtract_the_sum;
mod csv_representation_of_array;
mod alternative_case;
mod how_many_lightsabers;
mod convert_string_to_number;
mod squaring_an_argument;
mod will_you_make_it;
mod parse_nice_int_from_char;
mod htmlspecialchars;
mod difference_of_volumes_of_cuboids;
mod third_angle_of_a_triangle;
mod calculate_bmi;
mod beginner_reduce_but_grow;
mod switch_it_up;
mod string_repeat;
mod grasshopper_basic_function_fixer;
mod keep_hydrated;
mod function_1_hello_word;
mod opposites_attract;
mod simple_multiplication;
mod reversed_strings;
mod grasshopper_messi_goals;
mod remove_first_and_last_char;
mod get_mean_of_array;
mod welcome;
mod are_you_playing_banjo;
mod abbreviate_two_word_name;
mod count_the_monkeys;
mod gravity_flip;
mod beginner_series_2_clock;
mod coefficients_quadratic_equation;
mod basic_mathematical_operations;
mod even_or_odd;
mod you_cant_code_under_pressure_1;
mod count_positives_sum_negatives;
mod count_odd_numbers_below_n;
mod the_if_function;
mod object_oriented_piracy;
mod makeuppercase;
mod century_from_year;
mod sum_of_differences_in_array;
mod calculate_average;
mod will_there_be_enough_space;
mod volume_of_cuboid;
mod holiday_viii_duty_free;
mod thinkful_logic_drills_traffic_light;
mod beginner_lost_without_map;
mod beginner_series_4_cockroach;
mod total_amount_of_points;
mod filling_an_array_part_1;
mod grasshopper_summation;
mod counting_sheep;
mod holiday_vi_shark_pontoon;
mod feast_of_many_beasts;
mod find_nearest_square_number;
mod multiplication_table_for_number;
mod how_much_i_love_you;
mod expression_matters;
mod grasshopper_terminal_game_move;
mod removing_elements;
mod dna_to_rna_conversion;
mod wilson_primes;
mod convert_boolean_values_to_string;
mod quarter_of_the_year;
mod remove_string_spaces;
mod find_first_non_consecutive_number;
mod returning_strings;
mod nth_power;
mod is_n_divisible_by_x_and_y;
mod just_count_sheep;
mod grasshopper_if_else_syntax_debug;
mod grasshopper_messi_goals_function;
mod square_or_square_root;
mod grasshopper_grade_book;
mod beginner_series_1_school_paperwork;
mod array_plus_array;
mod grasshopper_personalized_message;
mod reversed_words;
mod do_i_get_a_bonus;
mod get_planet_name_by_id;
mod grasshopper_check_for_factor;
mod opposite_number;
mod count_by_x;
mod convert_string_to_array;
mod bin_to_decimal;
mod what_is_between;
mod convert_number_to_reversed_array_of_digits;
mod sort_and_star;
mod sum_of_postives;
mod contamination_1_string;
mod name_shuffler;

#[cfg(test)]

fn minimum(arr: &[i32]) -> i32 {
    let mut min = i32::MAX;

    for i in 0..arr.len() {
        if arr[i] < min {
            min = arr[i];
        }
    }
    return min;
}

fn maximum(arr: &[i32]) -> i32 {
    let mut max = i32::MIN;

    for i in 0..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }
    return max;
}
mod tests {
    use super::{minimum, maximum};

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected_min: i32, expected_max: i32) {
        assert_eq!(minimum(arr), expected_min, "{ERR_MSG} with function minimum and arr = {arr:?}");
        assert_eq!(maximum(arr), expected_max, "{ERR_MSG} with function maximum and arr = {arr:?}");
    }

    #[test]
    fn fixed_tests() {
        dotest(&[-52, 56, 30, 29, -54, 0, -110], -110, 56);
        dotest(&[42, 54, 65, 87, 0], 0, 87);
        dotest(&[1, 2, 3, 4, 5, 10], 1, 10);
        dotest(&[-1, -2, -3, -4, -5, -10, 534, 43, 2, 1, 3, 4, 5, 5, 443, 443, 555, 555], -10, 555);
        dotest(&[9], 9, 9);
        dotest(&[4,6,2,1,9,63,-134,566], -134, 566);
    }
}

