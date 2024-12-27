use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (p1, p2) = input.split("\r\n\r\n").next_tuple().unwrap();

    // println!("p1:\n{}", p1);
    // println!("p2:\n{}", p2);
    let page_ordering_rules = p1
        .lines()
        .map(|line| {
            line.split('|')
                .map(|s| s.parse::<u64>().unwrap())
                .rev()
                .next_tuple::<(_, _)>()
                .unwrap()
        })
        .into_grouping_map()
        .collect::<HashSet<_>>();

    // println!("rules:\n{:?}", &page_ordering_rules);
    let updates = p2.lines().map(|line| {
        line.split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });
    let result: u64 = updates
        .filter(|update| is_in_correct_order(&update, &page_ordering_rules))
        .map(|update| *update.get(update.len().div_euclid(2)).unwrap())
        .sum();

    println!("part 1: {}", result);
}

fn is_in_correct_order<T>(update: &[T], rules: &HashMap<T, HashSet<T>>) -> bool
where
    T: Eq + Hash + Copy,
{
    let contained_pages: HashSet<_> = update.iter().map(|p| *p).collect();
    let mut prev_pages = HashSet::new();
    for page in update.iter() {
        if let Some(must_come_before) = rules.get(page) {
            if !must_come_before
                .intersection(&contained_pages)
                .all(|p| prev_pages.contains(p))
            {
                return false;
            }
        }

        prev_pages.insert(*page);
    }

    return true;
}
