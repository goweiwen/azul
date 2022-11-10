use std::cmp::min;
use std::fmt::{Display, Formatter, Write};

use anyhow::{bail, Context, ensure, Result};
use rand::Rng;
use strum::IntoEnumIterator;

use strum_macros::EnumIter;

const FACTORY_COUNT: [u32; 4] = [3, 5, 7, 9];
const OVERFLOW_COST: [u32; 7] = [-1, -1, -2, -2, -2, -3, -3];
const OVERFLOW_CUMULATIVE_COST: [u32; 7] = {
    OVERFLOW_COST.iter().scan(0, |acc, &x| {
        *acc = *acc + x;
        Some(*acc)
    }).collect()
};

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum PlayerColor {
    White,
    Black,
    Gray,
    Brown,
}

impl Display for PlayerColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            PlayerColor::White => '1',
            PlayerColor::Black => '2',
            PlayerColor::Gray => '3',
            PlayerColor::Brown => '4',
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Blue,
    Yellow,
    Red,
    Black,
    Teal,
    FirstPlayer,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Tile::Blue => 'B',
            Tile::Yellow => 'Y',
            Tile::Red => 'R',
            Tile::Black => 'K',
            Tile::Teal => 'T',
            Tile::FirstPlayer => '1',
        })
    }
}

pub struct Factory(Vec<Tile>);

impl Display for Factory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.iter().map(ToString::to_string).join())
    }
}

impl Factory {
    fn fill(&mut self, bag: &mut Bag) -> Result<()> {
        self.0 = bag.take(4).collect();
        Ok(())
    }
}

pub struct Bag(Vec<Tile>);

impl Bag {
    fn shuffle(&mut self, rng: impl Rng) {
        rand::shuffle(&mut self.0, rng);
    }

    fn take(&mut self, n: u32) -> Result<Vec<Tile>> {
        let mut tiles = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let tile = tiles.pop().context("ran out of tiles")?;
            tiles.push(tile);
        }
        Ok(tiles)
    }
}

#[derive(Debug, Clone)]
pub struct Wall([[Tile; 5]; 5]);

impl Wall {}

impl Display for Wall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..5 {
            f.write_str(self.0[i].iter().map(ToString::to_string).join())?;
        }
        Ok(())
    }
}

pub struct PatternLines(Vec<Vec<Tile>>);

impl PatternLines {}

impl Display for PatternLines {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..5 {
            f.write_str(self.0[i].iter().map(ToString::to_string).join())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FloorLine(Vec<Tile>);

impl FloorLine {
    fn add(&mut self, tile: Tile) -> Result<()> {
        ensure!(self.0.len() < OVERFLOW_CUMULATIVE_COST.len(),
            "overflow exceeded maximum of {} tiles", OVERFLOW_CUMMULATIVE_COST.len());
        self.0.push(tile);
        Ok(())
    }

    fn points(&self) -> u32 {
        OVERFLOW_CUMULATIVE_COST[min(self.0.len(), OVERFLOW_CUMULATIVE_COST.len() - 1)]
    }
}

impl Display for FloorLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.iter().map(ToString::to_string).join())
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    color: Color,
    points: u32,
    wall: Wall,
    pattern_lines: PatternLines,
    floor_line: FloorLine,
}

impl Player {
    fn new(color: PlayerColor) -> Self {
        Self {
            color,
            points: 5,
            wall: Wall::default(),
            pattern_lines: PatternLines::default(),
            floor_line: FloorLine::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    bag: Bag,
    players: Vec<Player>,
    factories: Vec<Factory>,
    center: Vec<Tile>,
}

impl Game {
    fn new(players: u32) -> Self {
        ensure!(players <= 4, "maximum of 4 players");
        let players = PlayerColor::iter().take(players as usize).map(|color| Player::new(color));
        let factories = (0..FACTORY_COUNT[players]).iter().map(|| Factory::new()).collect();
        Game {
            bag: Bag::new(),
            players,
            factories,
            center: vec![],
        }
    }

    fn deal(&mut self) {
        self.factories.iter_mut().for_each(|factory| factory.fill(&mut self.bag)?);
    }
}