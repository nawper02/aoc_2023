// day 03


use std::iter::Enumerate;
use std::str::Chars;

fn main() {
    let input = include_str!("input.txt");

    let result01 = part01(input);
    match result01{
        Ok(value) => println!("Part 01 answer is: {}", value),
        Err(e) => println!("Part 01 Error: {}", e),
    }

    //let result02 = part02(input);
    //match result02{
    //    Ok(value) => println!("Part 02 answer is: {}", value),
    //    Err(e) => println!("Part 02 Error: {}", e),
    //}
}

fn part01(input: &str) -> Result<String, String> {
    // double nested loop to loop over whole thing by coordinate ✓
    // if we find a digit, look forward to find the rest of the number ✓
    // look at all neighboring cells for a symbol. if we find one, add the number to the sum ✓

    let mut sum: i32 = 0;
    let mut numbers: Vec<Number> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut chars_iter = line.chars().enumerate();
        while let Some((j, char)) = chars_iter.next() {
            if char.is_digit(10) {
                numbers.push(catch_number(i, j, &mut chars_iter, char));
            }
        }
    }

    for number in numbers {
        for i in (if number.coordinate_start.0 > 0 { number.coordinate_start.0 - 1 } else { 0 })..=(number.coordinate_end.0 + 1) {
            for j in (if number.coordinate_start.1 > 0 { number.coordinate_start.1 - 1 } else { 0 })..=(number.coordinate_end.1 + 1) {
                if let Some(char) = get_char_at(input, i, j) {
                    if !(char.is_digit(10) || (char == '.')) {
                        sum += number.number_val;
                    }
                }
            }
        }
    }


    Ok(sum.to_string())
}

fn catch_number(i: usize, j: usize, chars_iter: &mut Enumerate<Chars>, current_char: char) -> Number {

    let number_str_partial: String = chars_iter
        .by_ref() // to prevent move
        .take_while(|&(_, c)| c.is_digit(10)) // _ is because each item in iter is (j, c)
        .map(|(_, c)| c) // extract just the character from each item
        .collect();

    let number_str = format!("{current_char}{number_str_partial}");
    let number_val = number_str.parse::<i32>().unwrap();
    let coordinate_start = (i, j);
    let coordinate_end = (i, j + number_str.chars().count() - 1);

    println!("{:?}: Start {:?} End {:?}", number_str, coordinate_start, coordinate_end);
    Number{number_str, number_val, coordinate_start, coordinate_end}
}

fn get_char_at(input: &str, i: usize, j: usize) -> Option<char> {
    input.lines().nth(i)?.chars().nth(j)
}

struct Number {
    number_str: String,
    number_val: i32,
    coordinate_start: (usize, usize),
    coordinate_end: (usize, usize),
}

fn part02(input: &str) -> Result<String, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part01_works() {
        assert_eq!("88", part01(include_str!("example_input.txt")).unwrap());
    }

    #[test]
    fn part02_works() {
        assert_eq!("888", part02(include_str!("example_input.txt")).unwrap());
    }
}

/*
             @@@@@@@@@@@@@%                        @@@@@@@.
        ,@@@@@@@@@@@@      @@@@.                         .,@@@,
      @@@@@@@@@@@@@@@          @@,                           .@@@.
    @@@@@@@@@@@@@@@@@            @@,                            @@@.
   @@@@@@@@@@@@@@@@@@             @@*         @@@@@@@@@@/         @@,
  @@@@@@@@@@@@@@@@@@@              @@,      @@@@@@@@*. .@@@        @@.
  @@ Kin Blandford @@              (@#.    @@@@@@@@@*.   @@*.      @@,
  @@ ------------- @@              *@%.@@, @@@@@@@@@*.   @@*.      @@*
  @@@@@@@@@@@@@@@@@@@              @@*.@@, @@@@@@@@@*.  @@@,       @@*
  @@@@@@@@@@@@@@@@@@@             @@#,@@*. @@*@@@@@@@@@@@*.        @@*
  @@@@@@@@@@@@@@@@@@@            @@/,@@*,  @@*   .@@/,.            @@*
  @@,.@@@@@@@@@@@@@@@          @@@*.@@*.   @@*    @@*.             @@*
  @@,  .@@@@@@@@@@@@@       @@@*,(@@*,/@@@/*,..@@@@@@@@&           @@*
  @@,      ,@@@@@@@@@@@@@@@**.#@@@*,@@@*.@@@@/    @@@@@@@@@@@.     @@*
  @@,            ..@@,..  @@@@**. @@*,@@@         @@@@@@@@@@@@@@.  @@*
  @@,              @@,    @@,   @@/,@@@           @@@@@@@@@@@@@@@@,@@*
  @@,           @@@@@@@,  @@,  @@*.@@             @@@@@@@@@@@@@@@@@@@*
  @@,        @@@   @@@@@@@@@, @@*.@@              @@@@@@@@@@@@@@@@@@@*
  @@,       @@     @@@@@@@@@, @@*/@#              @@@@@@@@@@@@@@@@@@@*
  @@,      &@      @@@@@@@@@, .*.@@               @@@@@@@@@@@@@@@@@@@*
  @@,       @@     @@@@@@@@/.     @@              @@@@@@@@@@@@@@@@@@@,
   @@.       @@@   @@@@@@@*.      @@              @@@@@@@@@@@@@@@@@@*.
   .@@.        .,@@@@@(*,          @@.            @@@@@@@@@@@@@@@@@/,
     @@@.                           .@@           @@@@@@@@@@@@@@@@*.
       @@@,                           .@@@        @@@@@@@@@@@@@@*.
         .*@@@@@                         .@@@@@.  @@@@@@@@@@/*.
              .,,*/(,                         .,*//***,..
 */