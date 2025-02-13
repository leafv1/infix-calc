/*-----------------------------*/

#[derive(Copy, Clone)]
enum Op {
    Add, Sub, Mul, Div, Exp
}

impl Op {
    fn rank(&self) -> u32 {
        match self {
            Self::Add => 0,
            Self::Sub => 0,
            Self::Mul => 1,
            Self::Div => 1,
            Self::Exp => 2,
        }
    }

    fn higher(&self, other: &Self) -> bool {
        if let (Self::Exp, Self::Exp) = (self, other) {
            true
        } else {
            self.rank() > other.rank()
        }
    }

    fn perform(&self, a: f64, b: f64) -> f64 {
        match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
            Self::Exp => f64::powf(a, b),
        }
    }
}

/*-----------------------------*/

enum Token {
    Invalid,
    Empty,
    Open,
    Close,
    Num(f64),
    Op(Op),
}

impl Token {
    fn get(mut s: &str) -> (Self, &str) {
        s = s.trim_start();

        if let Ok((x, n)) = fast_float::parse_partial::<f64, _>(s) {
            return (Self::Num(x), &s[n..]);
        }

        let t = match s.chars().next() {
            Some(c) => match c {
                '(' => Self::Open,
                ')' => Self::Close,
                '+' => Self::Op(Op::Add),
                '-' => Self::Op(Op::Sub),
                '*' => Self::Op(Op::Mul),
                '/' => Self::Op(Op::Div),
                '^' => Self::Op(Op::Exp),
                _ => return (Self::Invalid, s)
            },
            None => return (Self::Empty, s)
        };

        (t, &s[1..])
    }
}

/*-----------------------------*/
struct Pair {
    num: f64,
    op: Option<Op>
}

impl Pair {
    fn sentinel() -> Self {
        Self { num: 0.0, op: Some(Op::Add) }
    }
}

struct Base {
    idx: usize,
    parens: u32
}

fn solve(mut s: &str) -> Result<f64, ()> {
    let mut pairs: Vec<Pair> = Vec::new();
    let mut bases: Vec<Base> = Vec::new();

    let mut r = Pair::sentinel();

    let mut base = Base { idx: 0, parens: 1 };

    'advance: loop {
        let mut t;

        let mut l = r;

        let mut nopen: u32 = 0;
        let num = loop {
            (t, s) = Token::get(s);

            match t {
                Token::Open => nopen += 1,
                Token::Num(x) => break x,
                _ => break 'advance Err(()),
            }
        };

        let mut nclose: u32 = 0;
        let op = loop {
            (t, s) = Token::get(s);

            match t {
                Token::Close => nclose += 1,
                Token::Op(x) => break Some(x),
                Token::Empty => {
                    nclose += 1;
                    break None
                },
                _ => break 'advance Err(()),
            }
        };

        r = Pair { num, op };

        if nopen > 0 {
            pairs.push(l);
            bases.push(base);

            base = Base { idx: pairs.len(), parens: nopen };
            l = Pair::sentinel();
        }

        'fold: loop {
            if let (true, Some(rop)) = (nclose == 0, r.op) {
                if rop.higher(&l.op.unwrap()) {
                    pairs.push(l);
                    continue 'advance;
                }
            }

            r.num = l.op.unwrap().perform(l.num, r.num);

            loop {
                if pairs.len() > base.idx {
                    l = pairs.pop().unwrap();
                    continue 'fold;
                }

                while base.parens > 0 {
                    if nclose == 0 {
                        continue 'advance;
                    }

                    nclose -= 1;
                    base.parens -= 1;
                }

                base = if let Some(b) = bases.pop() { b } else {
                    break 'advance (
                        if let (true, Token::Empty) = (nclose == 0, t) {
                            Ok(r.num)
                        } else { Err(()) }
                    )
                };
            }
        }
    }
}

/*-----------------------------*/

fn main() {
    let test_str =
        "100 - (((((((2 * (((100))))) / 4) \
        + 2 ^ (5 / ((2 + 2) * 2))))) ^ 2 ^ 2) * 0.00001";

    let exp = &std::env::args().nth(1).unwrap_or(test_str.to_string());

    println!("{}", solve(exp).expect("solve failed"));
}
