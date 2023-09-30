/* Find the Minimum Number of Coins Needed to Make Change */

use std::cmp;
use std::collections::HashMap;

pub fn recursive_change(money : u32 , max_idx:usize, coins : &Vec<u32>) -> u32 {
    if money <= 0 {
        return 0
    } else if max_idx >= coins.len() {
        return u32::MAX
    }

    if money == coins[max_idx] {
        // println!("{}", coins[max_idx]);
        return 1
    } else if money < coins[max_idx] {
        return recursive_change(money, max_idx+1, coins)
    } else {
        let c = money / coins[max_idx];
        let remain = money - coins[max_idx]*c;
        return cmp::min(c+recursive_change(remain, max_idx, coins), recursive_change(money, max_idx+1, coins))
    }
}

pub fn dp_remainder(money:u32, map: &mut HashMap<u32,u32>, coins: &Vec<u32>) -> u32{
    let n_coins = coins.len();
    for i in 0..n_coins {
        if coins[i] > money {
            continue
        } else if coins[i] == money {
            map.insert(money, 1);
        } else {
            let remainder = money-coins[i];
            if !map.contains_key(&remainder) {
                let remainder_change = dp_remainder(remainder, map, &coins);
                map.insert(remainder, remainder_change);
            }
            if !map.contains_key(&money) {
                map.insert(money, 1+map[&remainder]);
            } else {
                *map.get_mut(&money).unwrap() = cmp::min(1+map[&remainder], map[&money]);
            }
        }
    }
    map[&money]
}

pub fn dp_change(money:u32, coins: &Vec<u32>) -> u32 {
    let mut change_map : HashMap<u32,u32> = HashMap::new();
    dp_remainder(money, &mut change_map, &coins);
    change_map[&money]
}

pub fn run(content: &Vec<String>) {
    let money = content[0].parse::<u32>().expect("money is not parsed to integer");
    let line = content[1].split(",");

    let mut coins = Vec::new();
    for coin in line {
        coins.push(coin.parse::<u32>().expect("coin is not parsed"));
    }
    coins.sort();
    coins.reverse();


    // let mut min = recursive_change(money, 0, &coins);
    let mut min = dp_change(money,&coins);
    println!("{min}");
}