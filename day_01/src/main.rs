// day 01

fn main() {
    let input = include_str!("input.txt");

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
    // start sum at 0
    let mut sum = 0;
    // loop over lines in input
    for line in input.lines() {
        // filter out all of the digits in the line
        let digits_only: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        // the number for the line is 10x the left ([0]) digit plus the right ([-1]) digit
        let num = digits_only[0].to_digit(10).unwrap() * 10 + digits_only[digits_only.len() - 1].to_digit(10).unwrap();
        //add the number for the line to the sum
        sum += num as i32;
    }
    Ok(sum.to_string())
}

fn part02(input: &str) -> Result<String, String> {
    // map words to number replacements. leave outer characters to prevent inner replacements
    // from breaking outer replacements
    let num_map = vec![
        ("one", "o1e"), ("two", "t2o"), ("three", "t3e"), ("four", "f4r"),
        ("five", "f5e"), ("six", "s6x"), ("seven", "s7n"), ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    // turn the input into a string so we can call .replace
    let mut input_string = input.to_string();
    // loop over the map. we take ownership here but thats fine since its not used anywhere else
    for (word, num) in num_map {
        // replace any number words with their corresponding digit
        input_string = input_string.replace(word, num);
    }

    // call part01 on the substituted text!
    part01(input_string.as_str())

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part01_works() {
        assert_eq!("142", part01(include_str!("example_input.txt")).unwrap());
    }

    #[test]
    fn part02_works() {
        assert_eq!("281", part02(include_str!("example_input.txt")).unwrap());
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