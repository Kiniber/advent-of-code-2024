
fn main() {
    println!("Ready for the Day02 challange!");
    let unusual_data = include_str!("../resources/data.csv");
    println!("The unusual data to validate: {unusual_data}");
    let reports = unusual_data.split("\n").map(|report|{
        report.split_whitespace().map(|word|word.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();
    println!("Total amount of reports: {}",reports.len());
    let reports_to_analyze = reports.clone();
    let valid_reports = reports_to_analyze.into_iter().filter(|report|validate_report(report)).collect::<Vec<Vec<i32>>>();
    println!("Valid report amount: {}",valid_reports.len());
    let dampening = 1;
    println!("Now with dampening={dampening}");
    let valid_reports = &reports.into_iter().filter(|report|report_unsafety_level(report) <= dampening).collect::<Vec<Vec<i32>>>();
    println!("Valid report amount with dampening: {}",valid_reports.len());
}

fn validate_report(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return false;
    }
    let mut previous_signum = 0;
    for i in 0..report.len()-1 {
        let n1 = report[i];
        let n2 = report[i+1];
        if n1 == n2 {
            return false; //staying on the same level is not allowed
        }
        let new_signum = (n1-n2).signum();
        if previous_signum != 0 && previous_signum != new_signum {
            return false; // the levels increased and then decreased or the other way round. It is not allowed
        }
        previous_signum = new_signum;
        match (n1-n2).abs() {
            diff if diff >=1 && diff <=3 => (),
            _ => return false
        }
    }
    true
}


fn report_unsafety_level(report: &Vec<i32>) -> usize {
    if report.len() < 2 {
        return report.len();
    }
    let mut unsafety_level = 0;
    let mut previous_signum = 0;

    for i in 0..report.len()-1 {
        let n1 = report[i];
        let n2 = report[i+1];
        if n1 == n2 {
            unsafety_level+=1; //staying on the same level is not allowed
            continue;
        }
        let new_signum = (n1-n2).signum();
        if previous_signum != 0 && previous_signum != new_signum {
            unsafety_level+=1; // the levels increased and then decreased or the other way round. It is not allowed
            continue;
        }
        previous_signum = new_signum;
        match (n1-n2).abs() {
            diff if diff >=1 && diff <=3 => (),
            _ => {
                unsafety_level+=1;
                continue;
            }
        }
    }
    unsafety_level
}