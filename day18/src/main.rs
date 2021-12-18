use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type Input = NumRef;
type Output = u64;

#[derive(Debug)]
enum Value {
    Num(u8),
    Pair(NumRef, NumRef),
}

enum Parentage {
    Root,
    LeftChild(NumRef),
    RightChild(NumRef),
}

impl std::fmt::Debug for Parentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parentage")
    }
}

#[derive(Debug)]
struct Number {
    val: Value,
    parent: Parentage,
}

type NumRef = Rc<RefCell<Number>>;

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => n.fmt(f),
            Self::Pair(l, r) => write!(f, "[{},{}]", l.borrow().val, r.borrow().val),
        }
    }
}

impl Number {
    fn rc(self) -> NumRef {
        Rc::new(RefCell::new(self))
    }

    fn value(n: u8) -> NumRef {
        Self {
            val: Value::Num(n),
            parent: Parentage::Root,
        }
        .rc()
    }

    // Interior mutability has defeated me.
    fn true_clone(&self) -> NumRef {
        parse(&self.val.to_string())
    }

    fn depth(&self) -> u8 {
        match &self.parent {
            Parentage::Root => 0,
            Parentage::LeftChild(n) | Parentage::RightChild(n) => 1 + n.borrow().depth(),
        }
    }

    fn pair(l: NumRef, r: NumRef) -> NumRef {
        let this = Self {
            val: Value::Num(0),
            parent: Parentage::Root,
        }
        .rc();

        l.borrow_mut().parent = Parentage::LeftChild(Rc::clone(&this));
        r.borrow_mut().parent = Parentage::RightChild(Rc::clone(&this));

        this.borrow_mut().val = Value::Pair(l, r);

        this
    }

    fn is_pair(&self) -> bool {
        matches!(&self.val, Value::Pair(..))
    }

    fn is_num(&self) -> bool {
        matches!(&self.val, Value::Num(..))
    }

    fn is_num_pair(&self) -> bool {
        if !self.is_pair() {
            return false;
        }
        let (l, r) = self.as_pair();
        let lb = l.borrow().is_num();
        let rb = r.borrow().is_num();
        lb && rb
    }

    fn as_num(&self) -> u8 {
        match &self.val {
            Value::Num(n) => *n,
            _ => panic!(),
        }
    }

    fn as_num_mut(&mut self) -> &mut u8 {
        match &mut self.val {
            Value::Num(n) => n,
            _ => panic!(),
        }
    }

    fn as_pair(&self) -> (NumRef, NumRef) {
        match &self.val {
            Value::Pair(l, r) => (l.clone(), r.clone()),
            _ => panic!(),
        }
    }

    fn explode(this: NumRef) {
        let (l, r) = this.borrow().as_pair();

        let new_val = Value::Num(0);
        this.borrow_mut().val = new_val;

        let l = l.borrow().as_num();
        let r = r.borrow().as_num();

        if let Some(n) = Self::left_neighbor(this.clone()) {
            *n.borrow_mut().as_num_mut() += l;
        }

        if let Some(n) = Self::right_neighbor(this.clone()) {
            *n.borrow_mut().as_num_mut() += r;
        }
    }

    fn split(this: NumRef) {
        let n = this.borrow().as_num();
        let l = n / 2;
        let r = l + (n & 1);

        let lv = Self {
            parent: Parentage::LeftChild(this.clone()),
            val: Value::Num(l),
        };

        let rv = Self {
            parent: Parentage::RightChild(this.clone()),
            val: Value::Num(r),
        };

        this.borrow_mut().val = Value::Pair(lv.rc(), rv.rc());
    }

    fn left_neighbor(mut node: NumRef) -> Option<NumRef> {
        loop {
            let new_node;
            match &node.borrow().parent {
                Parentage::Root => return None,
                Parentage::LeftChild(p) => new_node = p.clone(),
                Parentage::RightChild(p) => {
                    let (l, _) = p.borrow().as_pair();
                    return Some(Self::rightmost_child(l));
                }
            }
            node = new_node;
        }
    }

    fn rightmost_child(mut node: NumRef) -> NumRef {
        while node.borrow().is_pair() {
            let n = node.borrow().as_pair().1;
            node = n;
        }
        node
    }

    fn right_neighbor(mut node: NumRef) -> Option<NumRef> {
        loop {
            let new_node;
            match &node.borrow().parent {
                Parentage::Root => return None,
                Parentage::RightChild(p) => new_node = p.clone(),
                Parentage::LeftChild(p) => {
                    let (_, r) = p.borrow().as_pair();
                    return Some(Self::leftmost_child(r));
                }
            }
            node = new_node;
        }
    }

    fn leftmost_child(mut node: NumRef) -> NumRef {
        while node.borrow().is_pair() {
            let n = node.borrow().as_pair().0;
            node = n;
        }
        node
    }

    fn with_nodes<F: FnMut(NumRef)>(this: NumRef, f: &mut F) {
        f(this.clone());
        if this.borrow().is_pair() {
            let (l, r) = this.borrow().as_pair();
            Self::with_nodes(l, f);
            Self::with_nodes(r, f);
        }
    }

    fn reduce_step(this: NumRef) -> bool {
        let mut changed = false;
        Self::with_nodes(this.clone(), &mut |n| {
            if changed {
                return;
            }

            if n.borrow().depth() >= 4 && n.borrow().is_num_pair() {
                Self::explode(n);
                changed = true;
            }
        });
        Self::with_nodes(this.clone(), &mut |n| {
            if changed {
                return;
            }

            if n.borrow().is_num() && n.borrow().as_num() >= 10 {
                Self::split(n);
                changed = true;
            }
        });
        changed
    }

    fn reduce(this: NumRef) {
        while Self::reduce_step(this.clone()) {}
    }

    fn magnitude(&self) -> u64 {
        if self.is_num() {
            self.as_num() as u64
        } else {
            let (l, r) = self.as_pair();
            let lm = l.borrow().magnitude();
            let rm = r.borrow().magnitude();
            3 * lm + 2 * rm
        }
    }
}

fn parse_rec(s: &mut &str) -> NumRef {
    match s.bytes().next() {
        Some(c @ b'0'..=b'9') => {
            *s = &s[1..];
            Number::value(c - b'0')
        }
        Some(b'[') => {
            *s = &s[1..];

            let l = parse_rec(s);

            assert_eq!(s.bytes().next(), Some(b','));
            *s = &s[1..];

            let r = parse_rec(s);

            assert_eq!(s.bytes().next(), Some(b']'));
            *s = &s[1..];

            Number::pair(l, r)
        }
        _ => panic!("foo"),
    }
}

fn parse(mut s: &str) -> Input {
    parse_rec(&mut s)
}

fn final_sum(inp: &[Input]) -> NumRef {
    let mut val = inp[0].borrow().true_clone();
    Number::reduce(val.clone());
    for addend in &inp[1..] {
        val = Number::pair(val, addend.borrow().true_clone());
        Number::reduce(val.clone());
    }
    val
}

fn part1(inp: &[Input]) -> Output {
    let m = final_sum(inp).borrow().magnitude();
    m
}

fn part2(inp: &[Input]) -> Output {
    let mut n = 0;
    for i in 0..inp.len() {
        for j in 0..inp.len() {
            if i == j {
                continue;
            }
            let a = inp[i].borrow().true_clone();
            let b = inp[j].borrow().true_clone();
            n = n.max(part1(&[a, b]));
        }
    }
    n
}

util::register!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode() {
        fn check(before: &str, after: &str) {
            let node = parse(before);
            Number::reduce_step(node.clone());
            assert_eq!(node.borrow().val.to_string(), after);
        }

        check("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        check("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        check("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        check(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        check(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn test_magnitude() {
        fn check(num: &str, mag: u64) {
            let node = parse(num);
            assert_eq!(node.borrow().magnitude(), mag);
        }

        check("[9,1]", 29);
        check("[1,9]", 21);
        check("[[9,1],[1,9]]", 129);
        check("[[1,2],[[3,4],5]]", 143);
        check("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        check("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        check("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        check("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        check(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        );
    }

    #[test]
    fn test_reduce() {
        let node = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        assert!(Number::reduce_step(node.clone()));
        assert_eq!(
            node.borrow().val.to_string(),
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
        );
        assert!(Number::reduce_step(node.clone()));
        assert_eq!(
            node.borrow().val.to_string(),
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
        );
        assert!(Number::reduce_step(node.clone()));
        assert_eq!(
            node.borrow().val.to_string(),
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        );
        assert!(Number::reduce_step(node.clone()));
        assert_eq!(
            node.borrow().val.to_string(),
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        );
        assert!(Number::reduce_step(node.clone()));
        assert_eq!(
            node.borrow().val.to_string(),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
        assert!(!Number::reduce_step(node.clone()));
    }

    #[test]
    fn test_sum() {
        fn check<const N: usize>(initial: [&str; N], done: &str) {
            let nodes = initial.map(parse);
            assert_eq!(final_sum(&nodes).borrow().val.to_string(), done);
        }

        check(
            ["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );

        check(
            ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );

        check(
            ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        );
        check(
            [
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[2,9]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[4,2],2],6],[8,7]]",
            ],
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        )
    }

    #[test]
    fn test_my_sanity() {
        let a = parse("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]");
        let b = parse("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]");
        let c = Number::pair(a, b);
        Number::reduce(c.clone());
        assert_eq!(
            c.borrow().val.to_string(),
            "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]",
        );
        assert_eq!(c.borrow().magnitude(), 3993);
    }
}
