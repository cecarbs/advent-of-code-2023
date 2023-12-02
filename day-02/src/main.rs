use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: u32 = sum_of_game_ids((12, "red"),  (13, "green"), (14, "blue"), input);
    println!("{}", output);
}

fn sum_of_game_ids(color_1: (u32, &str), color_2: (u32, &str), color_3: (u32, &str), input: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut input_color_map: HashMap<&str, u32> = HashMap::new();
    input_color_map.insert(color_1.1, color_1.0);
    input_color_map.insert(color_2.1, color_2.0);
    input_color_map.insert(color_3.1, color_3.0);

    // println!("{:?}", input_color_map);
    for line in input.lines() {
        let mut color_map: HashMap<&str, u32> = HashMap::new();

        let mut split: Vec<&str> = line.split(": ").collect();
        let game_id: u32 = get_game_id_value(split[0]);
        // println!("{}", game_id);
        let round: &str = split[1];
        let game: Vec<&str> = round.split("; ").collect();

        for cubes in game {
            let color_pairs: Vec<&str> = cubes.split(", ").collect();

            for color_pair in color_pairs {
                let pair: Vec<&str> = color_pair.split(' ').collect();

                let color: &str = pair[1];
                let count: u32 = pair[0].parse().unwrap();

                let mut entry: &mut u32 = color_map.entry(color).or_insert(0);
                *entry += count;

            }

            // println!("{:?}", color_map);

        }

        let mut is_eligible: bool = true;
        for (color, count) in color_map.iter() {
            if input_color_map.get(color).unwrap() <= count {
                is_eligible = false;
                break;
            }
        }

        if is_eligible {
            sum += game_id
        }

    }

    sum
}

fn get_game_id_value(game_id: &str) -> u32 {
    let split: Vec<&str> = game_id.split(' ').collect();
    let id: &str = split[1];

    id.parse().unwrap()
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
