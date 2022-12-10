use crate::tree::Tree;
use std::ops::Index;
use std::str::Lines;

type TreeLocation = (usize, usize);

pub struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    pub fn iter_trees(&self) -> impl Iterator<Item = TreeLocation> + '_ {
        self.trees
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
    }

    pub fn is_visible(&self, loc: &TreeLocation) -> bool {
        self.is_edge(loc)
            || self.is_visible_direction(loc, self.iter_left(loc))
            || self.is_visible_direction(loc, self.iter_right(loc))
            || self.is_visible_direction(loc, self.iter_up(loc))
            || self.is_visible_direction(loc, self.iter_down(loc))
    }

    pub fn get_scenic_score(&self, loc: &TreeLocation) -> usize {
        self.get_scenic_score_direction(loc, self.iter_left(loc))
            * self.get_scenic_score_direction(loc, self.iter_right(loc))
            * self.get_scenic_score_direction(loc, self.iter_up(loc))
            * self.get_scenic_score_direction(loc, self.iter_down(loc))
    }

    fn is_edge(&self, loc: &TreeLocation) -> bool {
        loc.0 == 0
            || loc.1 == 0
            || loc.1 == self.trees.len() - 1
            || loc.0 == self.trees[loc.1].len() - 1
    }

    fn is_visible_direction<T>(&self, loc: &TreeLocation, mut dir: T) -> bool
    where
        T: Iterator<Item = TreeLocation>,
    {
        let tree = &self[loc];
        dir.all(|other| &self[&other] < tree)
    }

    fn get_scenic_score_direction<T>(&self, loc: &TreeLocation, dir: T) -> usize
    where
        T: Iterator<Item = TreeLocation>,
    {
        let tree = &self[loc];
        let mut count = 0;
        for other in dir {
            count += 1;
            if &self[&other] >= tree {
                break;
            }
        }
        count
    }

    fn iter_left(&self, loc: &TreeLocation) -> impl Iterator<Item = TreeLocation> + '_ {
        let y = loc.1;
        (0..loc.0).rev().map(move |x| (x, y))
    }

    fn iter_right(&self, loc: &TreeLocation) -> impl Iterator<Item = TreeLocation> + '_ {
        let y = loc.1;
        ((loc.0 + 1)..self.trees[y].len()).map(move |x| (x, y))
    }

    fn iter_up(&self, loc: &TreeLocation) -> impl Iterator<Item = TreeLocation> + '_ {
        let x = loc.0;
        (0..loc.1).rev().map(move |y| (x, y))
    }

    fn iter_down(&self, loc: &TreeLocation) -> impl Iterator<Item = TreeLocation> + '_ {
        let x = loc.0;
        ((loc.1 + 1)..self.trees.len()).map(move |y| (x, y))
    }
}

impl Index<&TreeLocation> for Forest {
    type Output = Tree;

    fn index(&self, index: &TreeLocation) -> &Self::Output {
        &self.trees[index.1][index.0]
    }
}

impl From<Lines<'_>> for Forest {
    fn from(input: Lines<'_>) -> Self {
        let trees = input
            .map(|line| line.chars().map(Tree::from).collect())
            .collect();
        Forest { trees }
    }
}
