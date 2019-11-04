struct Die {
    value: i32,
}

struct Dice {
    dice: Vec<Die>,
}

impl Die {
    pub fn new() -> Die {
        Die {
            value: 1
        }
    }

    pub fn increase_by_one(&mut self) {
        if self.value < 6 {
            self.value += 1;
        }
    }

    pub fn reset_to_one(&mut self){
        self.value = 1;
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl Dice {
    pub fn new(dice_amount: i32) -> Dice {
        let mut dice: Vec<Die> = Vec::new();
        while dice.len() < dice_amount as usize {
            dice.push(Die::new());
        }

        Dice{
            dice
        }
    }

    pub fn get_dice_total(&self) -> i32 {
        self.dice.iter().map(|die| die.value).sum()
    }

    pub fn next_poss(&mut self) {
        let dice_amount = self.dice.len();

        for dice_number in 0..dice_amount {
            if self.dice[dice_number as usize].get_value() < 6 {
                self.dice[dice_number as usize].increase_by_one();
                break
            } else {
                self.dice[dice_number as usize].reset_to_one();
            }
        }
    }
}

fn rolldice_sum_prob(sum:i32, dice_amount:i32) -> f64 {
    // first make set of dice
    let mut set = Dice::new(dice_amount);

    // find number of possibilities
    let mut no_of_possibilities = 0;

    // set this while so that the program doesn't loop forever
    while set.get_dice_total() < dice_amount * 6 {
        if set.get_dice_total() == sum {
            no_of_possibilities += 1;
        }
        set.next_poss();
    }

    // now that I have the number of possible combinations, find the probability
    no_of_possibilities as f64 / (6_i32.pow(dice_amount as u32)) as f64
}
