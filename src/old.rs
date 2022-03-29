
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    None,
    SubTwo,
    MulThree,
    AddFive,
}

/*
#[derive(Debug, Clone, Copy)]
struct Entry {
    op: Operator,
    worked: bool,
    previous: u128,
}

impl Entry {

    const fn empty() -> Self {
        Entry {
            op: Operator::None,
            worked: false,
            previous: 0,
        }
    }

    fn put(& mut self, op: Operator, previous: u128) {
        self.op = op;
        self.previous = previous;
    }

}

const LENGTH: usize = 1024 * 1024;


fn round(map: & mut [Entry]) {
    let len = map.len();

    for i in 0..len {
        if i == 0 || !map[i].worked && map[i].op != Operator::None {

            if i > 2 {
                if map[i - 2].op == Operator::None {
                    map[i - 2].put(Operator::SubTwo, i as u128);
                }
            }

            if i * 3 < len && i != 0 {
                if map[i * 3].op == Operator::None {
                    map[i * 3].put(Operator::MulThree, i as u128);
                }
            }

            if i + 5 < len {
                if map[i + 5].op == Operator::None {
                    map[i + 5].put(Operator::AddFive, i as u128);
                }
            }

            map[i].worked = true;


        }
    }
}*/

fn step(i: u128) -> (Operator, u128) {

    use Operator::{None, SubTwo, AddFive, MulThree};

    static _1: [(Operator, u128); 90] = [(None, 0), (SubTwo, 3), (SubTwo, 4), (SubTwo, 5), (SubTwo, 6), (AddFive, 0), (SubTwo, 8), (SubTwo, 9), (SubTwo, 10), (MulThree, 3),
        (AddFive, 5), (SubTwo, 13), (SubTwo, 14), (SubTwo, 15), (AddFive, 9), (MulThree, 5), (SubTwo, 18), (SubTwo, 19), (SubTwo, 20), (AddFive, 14),
        (AddFive, 15), (SubTwo, 23), (SubTwo, 24), (SubTwo, 25), (MulThree, 8), (AddFive, 20), (SubTwo, 28), (MulThree, 9), (SubTwo, 30), (AddFive, 24),
        (MulThree, 10), (SubTwo, 33), (AddFive, 27), (SubTwo, 35), (AddFive, 29), (AddFive, 30), (SubTwo, 38), (AddFive, 32), (SubTwo, 40), (MulThree, 13),
        (AddFive, 35), (SubTwo, 43), (MulThree, 14), (SubTwo, 45), (AddFive, 39), (MulThree, 15), (SubTwo, 48), (AddFive, 42), (SubTwo, 50), (AddFive, 44),
        (AddFive, 45), (SubTwo, 53), (AddFive, 47), (SubTwo, 55), (MulThree, 18), (AddFive, 50), (SubTwo, 58), (MulThree, 19), (SubTwo, 60), (AddFive, 54),
        (MulThree, 20), (SubTwo, 63), (AddFive, 57), (SubTwo, 65), (AddFive, 59), (AddFive, 60), (SubTwo, 68), (AddFive, 62), (SubTwo, 70), (MulThree, 23),
        (AddFive, 65), (SubTwo, 73), (MulThree, 24), (SubTwo, 75), (AddFive, 69), (MulThree, 25), (SubTwo, 78), (AddFive, 72), (SubTwo, 80), (AddFive, 74),
        (AddFive, 75), (MulThree, 27), (AddFive, 77), (SubTwo, 85), (MulThree, 28), (AddFive, 80), (AddFive, 81), (MulThree, 29), (SubTwo, 90), (AddFive, 84),];

    static _2: [(Operator, u128); 15] = [(MulThree, 1), (AddFive, 2), (AddFive, 3), (SubTwo, 5), (AddFive, 5), (AddFive, 6), (MulThree, 3), (AddFive, 8), (SubTwo, 10), (MulThree, 4), (AddFive, 11), (AddFive, 12), (MulThree, 5), (SubTwo, 15), (AddFive, 15)];

    return if i < 90 {
        _1[i as usize]
    } else {
        let i = i - 90;

        let (o, offset) = _2[(i % 15) as usize];

        let (off, mul) = match o {
            None => panic!("wut"),
            SubTwo => (90, 15),
            AddFive => (84, 15),
            MulThree => (29, 5),
        };

        (o, off + (i / 15) * mul + offset)
    }


}

pub fn walk(i: u128) -> Vec<Operator> {

    if i == 0 {
        return vec![];
    }

    let mut v = Vec::new();

    let (mut op, mut prev) = step(i);

    while prev != 0 {
        v.insert(0, op);
        let (o, p) = step(prev);
        op = o; prev = p;
    }

    v.insert(0, op);

    v
}

pub fn evaluate(list: Vec<Operator>) -> u128 {
    let mut val = 0;

    for op in list {
        match op {
            Operator::None => {panic!("pokpok")}
            Operator::SubTwo => {val = val - 2;}
            Operator::MulThree => {val = val * 3;}
            Operator::AddFive => {val = val + 5;}
        }
    }

    val
}

pub fn verify(i: u128) -> bool {
    let list = walk(i);
    evaluate(list) == i
}