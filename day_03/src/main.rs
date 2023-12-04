// day 03


use std::collections::HashMap;
use std::iter::Enumerate;
use std::str::Chars;

fn main() {
    let input = include_str!("input.txt");

    let result01 = part01(input);
    let numbers = match result01{
        Ok((part01_answer, numbers)) => {println!("Part 01 answer is: {}", part01_answer); numbers}
        Err(e) => {println!("Part 01 Error: {}", e); Vec::new()}
    };

    let result02 = part02(input, numbers);
    match result02{
        Ok(value) => println!("Part 02 answer is: {}", value),
        Err(e) => println!("Part 02 Error: {}", e),
    }
}

fn part01(input: &str) -> Result<(String, Vec<Number>), String> {
    // double nested loop to loop over whole thing by coordinate ✓
    // if we find a digit, look forward to find the rest of the number ✓
    // look at all neighboring cells for a symbol. if we find one, add the number to the sum ✓

    let mut sum: i32 = 0;
    let mut numbers: Vec<Number> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut chars_iter = line.chars().enumerate();
        while let Some((j, char)) = chars_iter.next() {
            if char.is_digit(10) {
                //let mut n = ;
                numbers.push(catch_number(i, j, &mut chars_iter, char));
            }
        }
    }

    for number in &numbers {
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

    Ok((sum.to_string(), numbers))
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
    let coordinates: Vec<(usize, usize)> = std::iter::repeat(coordinate_start.0)
        .zip(coordinate_start.1..=coordinate_end.1)
        .collect();
    let mut neighbor_stars = Vec::new();
    Number{number_str, number_val, coordinate_start, coordinate_end, coordinates, neighbor_stars}
}

fn get_char_at(input: &str, i: usize, j: usize) -> Option<char> {
    input.lines().nth(i)?.chars().nth(j)
}

struct Number {
    number_str: String,
    number_val: i32,
    coordinate_start: (usize, usize),
    coordinate_end: (usize, usize),
    coordinates: Vec<(usize, usize)>,
    neighbor_stars: Vec<(usize, usize)>
}

fn part02(input: &str, mut numbers: Vec<Number>) -> Result<String, String> {
    // loop over characters until we find a *
    //
    // look around the * -- when we find a number, look in our list of numbers for the coordinate
    // to find which number it is. check all directions and make a collection of distinct numbers
    //
    // if there are two, multiply them and sum the results

    let directions: Vec<(i32, i32)> = vec![(1, 0),(0, 1),
                                           (-1, 0),(0, -1),
                                           (1, 1),(1, -1),
                                           (-1, 1),(-1, -1)];
    //     ...
    //     .*.      directions
    //     ...

    // numbers is no longer needed after this method so we can let it move into iter
    for mut number in numbers.iter_mut() {
        for coordinate in number.coordinates.iter() {
            for direction in &directions {
                // should've just not used usize in the first place...
                let (i, j) = ((coordinate.0 as i32 + direction.0) as usize, (coordinate.1 as i32 + direction.1) as usize);
                if let Some(char) = get_char_at(input, i, j) {
                    if char == '*' {
                        if !number.neighbor_stars.contains(&(i, j)) {
                            number.neighbor_stars.push((i, j))
                        }
                    }
                }
            }
        }
    }

    // we know the neighboring stars for each number.
    // now, we need to know how many times that star coordinate appears and in which numbers it appears.

    let mut star_map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for number in numbers {
        for star in number.neighbor_stars {
            star_map
                // get entry (vacant or occupied) for the star
                .entry(star)
                // if the entry doesnt exist, create it with an empty vector
                .or_insert_with(Vec::new)
                // if it does exist, push number_val to the vector
                .push(number.number_val)
        }
    }

    let mut sum = 0;

    for numbers in star_map.values() {
        if numbers.len() == 2 {
            sum += numbers.iter().product::<i32>();
        }
    }

    Ok(sum.to_string())
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
        assert_eq!(true, true);
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