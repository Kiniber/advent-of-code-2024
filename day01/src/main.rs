fn main() {
    let my_str = include_str!("../resources/input.csv");
    println!("We are attempting the challenge for day 1");
    println!("So here is our input list: \n{my_str}");
    let mut list1 = vec![];
    let mut list2 = vec![];
    my_str.split("\n").for_each(|line| {
            let mut next_part_iterator = line.split_whitespace();
            if let Some(first_part_str) = next_part_iterator.next() {
                let first_number: i32 = first_part_str.parse::<i32>().expect("Unable to parse the number");
                list1.push(first_number);
            } else {
                panic!("What? why? Is the input corrupt?")
            }
            if let Some(second_part_str) = next_part_iterator.next() {
                let second_number: i32 = second_part_str.parse::<i32>().expect("Unable to parse the number");
                list2.push(second_number);
            } else {
                panic!("What? why? Is the input corrupt?")
            }
        }
    );
    list1.sort();
    list2.sort();
    //println!("Here is the list1: \n{:?}",list1);
    //println!("Here is the list2: \n{:?}",list2);
    let mut iter2 = list2.iter();
    let result_list: Vec<i32> = list1.iter().map(|number1|(number1 - iter2.next().unwrap()).abs()).collect();
    println!("Here is the result: \n\n{:?}",result_list);
    let total = result_list.iter().sum::<i32>();
    println!("Here is the total distance: {total}");

    iter2 = list2.iter();
    let mut nextNumberOption2 = iter2.next();
    let similarity = list1.iter().map(|number1| {
        let mut counter = 0;
        while let Some(next_number2) = nextNumberOption2 {
            match next_number2 {
                n2 if n2 < number1 => {
                    nextNumberOption2 = iter2.next();
                }
                n2 if n2 == number1 => {
                    counter+=1;
                    nextNumberOption2 = iter2.next();
                },
                _ => {
                    break;
                }
            }
        }
        counter * number1
    }).sum::<i32>();
    println!("Here is the total similarity: {similarity}");
}
