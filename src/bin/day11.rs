
#[derive(Clone, Debug)]
enum WorryAdjustOperation {
    Add(u64),
    Mul(u64),
    Pow2,
}


#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: WorryAdjustOperation,
    test_divisible: u64,
    throws_to: (usize, usize)
}


fn parse_monkeys_from_txt(task_input: &str) -> Vec<Monkey> {
    let monkeys = task_input.split("\n\n")
    .map(|monkey| {
        let mut mk_iter = monkey.lines().skip(1);
        let (_, items) = mk_iter.next().unwrap().split_once(": ").unwrap();
        let items = items.split(", ").map(|i| i.parse().unwrap()).collect::<Vec<u64>>();

        let (_, operation) = mk_iter.next().unwrap().split_once("= ").unwrap();
        let operation = {
            let mut op_itr = operation.split(' ').skip(1);
            if op_itr.next().unwrap() == "+" {
                WorryAdjustOperation::Add(op_itr.next().unwrap().parse().unwrap())
            } else {
                let amount = op_itr.next().unwrap();
                if amount == "old" {
                    WorryAdjustOperation::Pow2
                } else {
                    WorryAdjustOperation::Mul(amount.parse().unwrap())
                }
            }
        };
        
        let (_, test_divisible) = mk_iter.next().unwrap().split_once("by ").unwrap();
        let test_divisible = test_divisible.parse().unwrap();
        
        let throws_to = {
            let (_, if_true) = mk_iter.next().unwrap().split_once("monkey ").unwrap();
            let (_, if_false) = mk_iter.next().unwrap().split_once("monkey ").unwrap();
            (if_true.parse::<usize>().unwrap(),
            if_false.parse::<usize>().unwrap())
        };

        Monkey{ items, operation, test_divisible, throws_to}
    })
    .collect::<Vec<Monkey>>();
    monkeys
}


fn keep_away_game(mut monkeys: Vec<Monkey>, rounds: usize, modulus: u64) -> u64 {
    let mut monkeys_inspected_times: Vec<u64> = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut mk = monkeys[i].clone();
            while let Some(item) = mk.items.pop() {
                let mut worry_level = match mk.operation {
                    WorryAdjustOperation::Add(n) => item + n,
                    WorryAdjustOperation::Mul(n) => item * n,
                    WorryAdjustOperation::Pow2 => item.pow(2)
                };

                worry_level = if modulus == 0 {worry_level / 3} else {worry_level % modulus};

                if worry_level % mk.test_divisible == 0 {
                    monkeys[mk.throws_to.0].items.push(worry_level);
                } else {
                    monkeys[mk.throws_to.1].items.push(worry_level);
                }
                monkeys[i].items.clear();
                monkeys_inspected_times[i] += 1;
            }
        }
    }
    monkeys_inspected_times.sort();
    monkeys_inspected_times.iter().rev().take(2).product()
}


fn main() {
    let instructions = include_str!("../../inputs/day11.in");

    let monkeys = parse_monkeys_from_txt(instructions);
    let modulus = monkeys.iter().map(|mk| mk.test_divisible).product::<u64>();
    println!("pt1 monkey business {}", keep_away_game(monkeys.clone(), 20, 0));
    println!("pt2 monkey business {}", keep_away_game(monkeys, 10000, modulus));
}
