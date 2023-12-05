// day xx


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

    // left | right
    for &(left, right) in game_parts.iter() {

        //small anonymous function to turn a string of space delimited numbers into ints
        let parse_numbers = |s: &str|
            s.split_whitespace()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

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