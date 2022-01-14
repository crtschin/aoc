use std::error::Error;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::Add;
use std::rc::Rc;

use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::{branch, character, combinator, sequence};

struct Depth(u8);
enum Node {
    Left(Depth, u8),
    Right(Depth, u8),
    Both(Depth, u8, u8),
}

enum Tree {
    Sub(Rc<Tree>, Rc<Tree>),
    Lit(u8),
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sub(arg0, arg1) => f.write_fmt(format_args!("<{:?}, {:?}>", arg0, arg1)),
            Self::Lit(arg0) => f.write_fmt(format_args!("{}", arg0)),
        }
    }
}

impl Add for Tree {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tree::Sub(Rc::new(self), Rc::new(other))
    }
}

impl Sum for Tree {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(None, |p: Option<Tree>, n| {
            match p {
                Some(t) => Some(t + n),
                None => Some(n),
            }
        }).unwrap_or(Tree::Lit(0))
    }
}

fn parse_tree(input: &str) -> nom::IResult<&str, Tree> {
    fn tree_choice(input: &str) -> nom::IResult<&str, Tree> {
        branch::alt((
            combinator::map_opt(digit1, |d: &str| d.parse().ok().map(Tree::Lit)),
            parse_tree,
        ))(input)
    }

    let tree: (&str, (Tree, Tree)) = nom::sequence::delimited(
        character::complete::char('['),
        sequence::separated_pair(tree_choice, character::complete::char(','), tree_choice),
        character::complete::char(']'),
    )(input)?;
    Ok((tree.0, Tree::Sub(Rc::new(tree.1 .0), Rc::new(tree.1 .1))))
}

fn parse_inputs() -> Result<Vec<Tree>, Box<dyn Error>> {
    let raw = include_str!("../../days/2021/day18.txt");
    let result = separated_list1(line_ending, parse_tree)(raw)?;
    Ok(result.1)
}

fn explode(tree: &Tree) -> (Tree, bool) {
    fn explode_rec(tree: &Tree) -> Result<Tree, (Option<u8>, Tree, Option<u8>)> {
        match tree {
            Tree::Sub(ref l, ref r) => {
                match (l, r) {
                    (Tree::Lit(_), Tree::Lit(_)) => todo!(),
                    _ => todo!()
                    // _ => {
                    //     match explode_rec(*l) {
                    //         Ok(tl) => match explode_rec(*r) {
                    //             Ok(tr) => Ok(Tree::Sub(Box::new(tl), Box::new(tr))),
                    //             Err(_) => todo!(),
                    //         },
                    //         Err(_) => todo!(),
                    //     }
                    // }
                }
            },
            Tree::Lit(_) => todo!(),
        }
    }
    todo!()
}

pub fn first() -> Result<i16, Box<dyn Error>> {
    let trees = parse_inputs()?;
    println!("{:?}", trees);
    println!("{:?}", trees.into_iter().sum::<Tree>());
    Ok(0)
}

pub fn second() -> Result<i16, Box<dyn Error>> {
    Ok(0)
}
