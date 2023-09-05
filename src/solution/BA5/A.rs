use std::cmp;

pub fn get_change(money : u8 , max_idx:usize, coins : &Vec<u8>) -> u8 {
    if max_idx >= coins.len() {
        return u8::MAX
    } else if money == 0 {
        return 0
    }

    if money == coins[max_idx] {
        println!("{}", coins[max_idx]);
        return 1
    } else if money < coins[max_idx] {
        return get_change(money, max_idx+1, coins)
    } else {
        let remain = money - coins[max_idx];
        return 1+ cmp::min(get_change(remain, max_idx, coins), get_change(remain, max_idx+1, coins))
    }
}

pub fn solve(money:u8, coins:&Vec<u8>) -> u8 {
    let idx = 0;

    let mut min = cmp::min(get_change(money, idx, coins), get_change(money, idx+1, coins));

    // println!("{min}");
    min
}



pub fn run(content: &Vec<String>) {
    let money = content[0].parse::<u8>().expect("money is not parsed to integer");
    let line = content[1].split(",");

    let mut coins = Vec::new();
    for coin in line {
        coins.push(coin.parse::<u8>().expect("coin is not parsed"));
    }
    coins.sort();
    coins.reverse();

    let n = solve(money, &coins);
    println!("{n}");
}