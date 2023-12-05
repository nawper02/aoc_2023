// day 04


fn main() {
    let input = include_str!("example_input.txt");

    let result01 = part01(input);
    match result01{
        Ok(value) => println!("Part 01 answer is: {}", value),
        Err(e) => println!("Part 01 Error: {}", e),
    }

    let result02 = part02(input);
    match result02{
        Ok(value) => println!("Part 02 answer is: {}", value),
        Err(e) => println!("Part 02 Error: {}", e),
    }
}

fn part01(input: &str) -> Result<String, String> {
    // vector of num matches for each card
    let mut cards : Vec<i32> = Vec::new();

    // take everything to the right of the colon for each line
    let game_parts: Vec<(&str,&str)> = input
        // get lines of input
        .lines()
        // take just everything after the :
        .map(|l| l.split_once(':').unwrap().1.trim())
        // turn it into a vector of (left side, right side)
        .map(|s| s.split_once('|').unwrap())
        .collect();

    //small anonymous function to turn a string of space delimited numbers into ints
    let parse_numbers = |s: &str|
        s.split_whitespace()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

    // left | right
    for &(left, right) in game_parts.iter() {

        let left_numbers: Vec<i32> = parse_numbers(left); // red underline: IDE bug
        let right_numbers: Vec<i32> = parse_numbers(right);

        // count how many numbers in left numbers are also in right numbers
        let n: i32 = left_numbers.iter()
            .filter(|&n| right_numbers.contains(n))
            .count() as i32;

        cards.push(n);

    }

    // 2^(n-1) points for each card (1 2 4 8 if starting at n=1), all cards added up
    Ok(
        cards
        .into_iter()
        .filter(|&n| n != 0)
        .map(|n| 2_i32.pow(n as u32 - 1))
        .sum::<i32>()
        .to_string()
    )
}

fn part02(input: &str) -> Result<String, String> {
    // vector of num matches for each card

    // take everything to the right of the colon for each line
    let mut game_parts: Vec<(&str,&str)> = input
        // get lines of input
        .lines()
        // take just everything after the :
        .map(|l| l.split_once(':').unwrap().1.trim())
        // turn it into a vector of (left side, right side)
        .map(|s| s.split_once('|').unwrap())
        .collect();

    //small anonymous function to turn a string of space delimited numbers into ints
    let parse_numbers = |s: &str|
        s.split_whitespace()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

    // left | right
    let mut i = 0;
    while i < game_parts.len() {
        let (left, right) = game_parts[i];

        let left_numbers: Vec<i32> = parse_numbers(left); // red underline: IDE bug
        let right_numbers: Vec<i32> = parse_numbers(right);

        // count how many numbers in left numbers are also in right numbers
        let n: i32 = left_numbers.iter()
            .filter(|&n| right_numbers.contains(n))
            .count() as i32;

        // we 'win' copies of the following cards.
        // n = 1: we win a copy of the next card.
        // n = 2: we win a copy of the next card and the one after it.
        // how....

        println!("{:?}", game_parts);

        // keep going until we process all the cards
        i += 1;
    }

    Ok(
        game_parts.iter().count().to_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part01_works() {
        assert_eq!("13", part01(include_str!("example_input.txt")).unwrap());
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