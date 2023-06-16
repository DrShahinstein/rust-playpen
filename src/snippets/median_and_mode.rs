use std::collections::HashMap;

pub fn calc() {
    let integer_list = [1, 2, 2, 3, 4, 10];
    let avg = find_average(&integer_list);
    let median = match find_median(&integer_list) {
        Some(value) => value.to_string(),
        None => "None".to_string(),
    };
    let mode = match find_mode(&integer_list) {
        Some(value) => value.to_string(),
        None => "None".to_string(),
    };

    let result = format!("Average: {:.2} Median: {} Mode: {}", avg, median, mode);
    println!("List: {:?}\n", integer_list);
    println!("Result:\n{result}");
}

fn find_average(list: &[i32]) -> f32 {
    list.iter().sum::<i32>() as f32 / list.len() as f32
}

fn find_median(list: &[i32]) -> Option<i32> {
    let mut sorted_list = list.to_vec();
    let len = sorted_list.len();

    if len == 0 {
        return None;
    }

    sorted_list.sort();
    let middle_index = len / 2;
    let middle_value = sorted_list[middle_index];

    if len % 2 == 0 {
        let second_middle_value = sorted_list[middle_index + 1];
        return Some((middle_value + second_middle_value) / 2);
    }

    Some(middle_value)
}

fn find_mode(list: &[i32]) -> Option<i32> {
    let count_map: HashMap<i32, u32> = list.iter().fold(HashMap::new(), |mut map, &num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });

    let max_count = count_map.values().max();

    match max_count {
        Some(&count) if count > 1 => {
            let modes: Vec<i32> = count_map
                .iter()
                .filter(|&(_, &c)| c == count)
                .map(|(&num, _)| num)
                .collect();

            Some(modes[0])
        }
        _ => None,
    }
}
