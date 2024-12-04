use regex::Regex;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Day 3, this seems like a regex Problem to me, so we use the regex crate");
    let input = include_str!("../resources/input.txt");
    //println!("This is the input:\n{input}");
    let regex_string = r"mul\((?<number1>\d{1,3}),(?<number2>\d{1,3})\)";
    println!("We try to solve it with this regex: {regex_string}");
    let regex = Regex::new(&regex_string)?;
    let sum = regex.captures_iter(input).map(|captured_part|{
        let number1 = captured_part.name("number1").unwrap().as_str().parse::<u32>().unwrap();
        let number2 = captured_part.name("number2").unwrap().as_str().parse::<u32>().unwrap();
        (number1, number2)
    }).map(|(number1, number2)| number1*number2).sum::<u32>();
    println!("The sum of products is {sum}");
    Ok(())
}

