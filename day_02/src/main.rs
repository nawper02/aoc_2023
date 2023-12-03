// day xx


use std::collections::HashMap;

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
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;
    let mut sum: i32 = 0;

    // loop over lines of input
    for line in input.lines() {
        // we initialize this here to start with no max values
        let mut max_values =  HashMap::new();

        // separate line into lhs (game and number) and rest of line
        let colon_split: Vec<&str> = line
            .split(':')
            .collect();

        // get the game number
        let game_num: &str = colon_split[0]
            .split_whitespace()
            .last()
            .unwrap();

        // get the quantity color pairs
        let parts: Vec<&str> = colon_split[1].split(|c| c == ',' || c == ';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        // part: "4 red"
        for part in parts {
            // for each pair, split into number and key.
            let mut split = part.split_whitespace();
            // if we can split each part into two strings,
            if let (Some(number_str), Some(color)) = (split.next(), split.next()) {
                // and if we can convert the number string into an integer,
                if let Ok(number) = number_str.parse::<i32>() {
                    max_values
                        // returns an entry that is either vacant or occupied
                        .entry(color)
                        // if occupied, replace it with the greater of the preexisting number or
                        // the new number.
                        .and_modify(|e| *e = i32::max(*e, number))
                        // if empty, insert the number (creating a color number pair)
                        .or_insert(number);
                }
            }
        }

        if (max_values["red"] < red_limit) & (max_values["green"] < green_limit) & (max_values["blue"] < blue_limit) {
            sum += game_num.parse::<i32>().unwrap();
        }
    }
    Ok(sum.to_string())
}

fn part02(input: &str) -> Result<String, String> {
    todo!()
}

fn part01_first_attempt(input: &str) -> Result<String, String> {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;
    let mut sum: i32 = 0;

    for line in input.lines() {
        let colon_split: Vec<&str> = line
            .split(':')
            .collect();
        let game_num: &str = colon_split[0]
            .split_whitespace()
            .last()
            .unwrap();
        // First try
        let possible = {
            let mut max_values =  HashMap::new();
            max_values.insert("red", 0);
            max_values.insert("green", 0);
            max_values.insert("blue", 0);

            let reveals: Vec<&str> = colon_split[1]
                .split(';')
                .map(|s| s.trim())
                .collect();
            for reveal in reveals {
                let reveal_colors: Vec<&str> = reveal.split(',').map(|s| s.trim()).collect();
                for q_c in reveal_colors {
                    let split: Vec<&str> = q_c.split_whitespace().collect();
                    let quantity = split[0].parse::<i32>().unwrap();
                    let color = split[1];
                    if let Some(value) = max_values.get_mut(color) {
                        *value = if quantity > *value { quantity } else { *value };
                    }
                    // surprised this one line replacement of the above 3 lines even compiles
                    //*max_values.get_mut(color).unwrap() = if quantity > *max_values.get_mut(color).unwrap() { quantity } else { *max_values.get_mut(color).unwrap() };
                }
            }
            (max_values["red"] < red_limit) & (max_values["green"] < green_limit) & (max_values["blue"] < blue_limit)
        };



        if possible {
            sum += game_num.parse::<i32>().unwrap();
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