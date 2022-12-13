use std::fs;
use core::slice::Iter;

#[derive(Debug, Clone)]
struct Stack {
    items: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn size(&self) -> usize {
        self.items.len()
    }

    fn push(&mut self, c: char) {
        self.items.push(c);
        // println!("Pushing {c} yielded {self:?}");
    }

    fn pop(&mut self) -> char {
        // println!("Popping from {:?}.", self.items);
        self.items.pop().unwrap()
    }

    fn top(&self) -> char {
        *(self.items.last().unwrap())
    }
}

#[derive(Debug, Clone)]
struct Stacks {
    stacks: Vec<Stack>
}

impl From<&str> for Stacks {
    fn from(s: &str) -> Self {
        let mut lines: Vec<&str> = s
            .split('\n')
            .collect();
        let num_stacks = lines[lines.len()-1].split_ascii_whitespace().count();
        let mut stacks: Vec<Stack> = (0..num_stacks).map(|_| Stack::new()).collect();
        
        lines.pop();
        lines.reverse();
        for l in lines {
            let bs = l.as_bytes();
            for s in 0..num_stacks {
                let pos = 1 + 4 * s;
                let c = bs[pos] as char;
                if c != ' ' {
                    stacks[s].push(c);
                }
            }
        }

        Stacks { stacks }
    }
}

impl Stacks {
    fn pop_from(&mut self, stack_num: u8) -> char {
        self.stacks[(stack_num - 1) as usize].pop()
    }

    fn push_to(&mut self, stack_num: u8, crate_val: char) {
        self.stacks[(stack_num - 1) as usize].push(crate_val);
    }

    fn take_from(&mut self, stack_num: u8, num_crates: u8) -> Vec<char> {
        let stack_len = self.stacks[(stack_num-1) as usize].size();
        self.stacks[(stack_num-1) as usize]
            .items
            .drain(0..(stack_len-(num_crates as usize)))
            .collect()
    }

    fn add_to(&mut self, stack_num: u8, block: &[char]) {
        self.stacks[(stack_num-1) as usize]
            .items
            .extend(block);
    }

    fn move_crate(&mut self, start_stack: u8, destination_stack: u8) {
        let crate_val = self.pop_from(start_stack);
        self.push_to(destination_stack, crate_val);
    }

    fn tops(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.top())
            .collect()
    }
}

impl Stacks {
    fn perform(&mut self, action: Action) {
        for _ in 0..action.num_crates {
            self.move_crate(action.start_stack, action.destination_stack);
        }
    }

    fn perform_grouped(&mut self, action: Action) {
        let mut clone_of_self = self.clone();
        let block = clone_of_self.take_from(action.start_stack, action.num_crates);
        self.add_to(action.destination_stack, &block);
        println!("{:?}", self);
    }
}

#[derive(Debug, Clone)]
struct Action {
    num_crates: u8,
    start_stack: u8,
    destination_stack: u8,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        let num_crates = parts[1].parse::<u8>().unwrap();
        let start_stack = parts[3].parse::<u8>().unwrap();
        let destination_stack = parts[5].parse::<u8>().unwrap();
        Action {
            num_crates,
            start_stack,
            destination_stack,
        }
    }
}

#[derive(Clone)]
struct Actions {
    actions: Vec<Action>,
}

impl From<&str> for Actions {
    fn from(s: &str) -> Self {
        let actions = s.split('\n').map(Action::from).collect();
        Actions { actions }
    }
}

impl IntoIterator for Actions {
    type Item = Action;

    type IntoIter = std::vec::IntoIter<Action>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.into_iter()
    }
}

fn main() {
    let contents = fs::read_to_string("../inputs/day_05.input")
        .expect("Should have been able to read the file");

    let mut sections = contents.split("\n\n");
    let stacks: Stacks = Stacks::from(sections.next().unwrap());
    let actions = Actions::from(sections.next().unwrap());

    {
        let mut stacks = stacks.clone();

        println!("{:?}", stacks);

        for action in actions.clone() {
            stacks.perform(action);
            // println!("{:?}", stacks);
        }

        println!("{:?}", stacks);

        println!("The tops are {}.", stacks.tops());
    }

    {
        let mut stacks = stacks.clone();

        println!("{:?}", stacks);

        for action in actions {
            stacks.perform_grouped(action);
        }

        println!("{:?}", stacks);

        println!("The tops are {}.", stacks.tops());
    }
}
