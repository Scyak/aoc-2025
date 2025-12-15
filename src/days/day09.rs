use itertools::Itertools;
use std::cmp::{max, min};
use std::fmt::Display;

use crate::days::Day;

pub struct Day09;

const _EXAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[derive(Clone, Copy, Debug)]
struct Tile {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    start_x: f64,
    end_x: f64,
    start_y: f64,
    end_y: f64,
}

impl Edge {
    fn new(start: Tile, end: Tile) -> Edge {
        Edge {
            start_x: f64::min(start.x as f64, end.x as f64),
            end_x: f64::max(start.x as f64, end.x as f64),
            start_y: f64::min(start.y as f64, end.y as f64),
            end_y: f64::max(start.y as f64, end.y as f64),
        }
    }

    fn is_point_on_edge(&self, x: f64, y: f64) -> bool {
        y <= self.end_y && y >= self.start_y && x <= self.end_x && x >= self.start_x
    }

    // whether a ray from (0,y) to (x,y) intersects the edge
    // returns false if endpoint of ray is on edge
    fn ray_intersects_edge(&self, x: f64, y: f64) -> bool {
        x > self.end_x && y >= self.start_y && y <= self.end_y
    }

    // see https://stackoverflow.com/questions/9043805/test-if-two-lines-intersect-javascript-function
    // note: does not include intersections on end points
    fn intersects(&self, other: &Self) -> bool {
        let (a, b, c, d) = (self.start_x, self.start_y, self.end_x, self.end_y);
        let (p, q, r, s) = (other.start_x, other.start_y, other.end_x, other.end_y);
        let determinant = (c - a) * (s - q) - (r - p) * (d - b);
        if determinant == 0.0 {
            return false;
        } else {
            let lambda = ((s - q) * (r - a) + (p - r) * (s - b)) as f64 / determinant as f64;
            let gamma = ((b - d) * (r - a) + (c - a) * (s - b)) as f64 / determinant as f64;
            return (0_f64 < lambda && lambda < 1_f64) && (0_f64 < gamma && gamma < 1_f64);
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    from_y: f64,
    to_y: f64,
    from_x: f64,
    to_x: f64,
    size: isize,
}

impl Rectangle {
    fn new(corner_a: &Tile, corner_b: &Tile) -> Rectangle {
        // shrink corners a bit so edge intersection later on works
        Rectangle {
            from_y: min(corner_a.y, corner_b.y) as f64 + 0.5,
            to_y: max(corner_a.y, corner_b.y) as f64 - 0.5,
            from_x: min(corner_a.x, corner_b.x) as f64 + 0.5,
            to_x: max(corner_a.x, corner_b.x) as f64 - 0.5,
            size: rectangle_area(corner_a, corner_b),
        }
    }

    fn get_edges(&self) -> Vec<Edge> {
        vec![
            Edge {
                start_x: self.from_x,
                end_x: self.from_x,
                start_y: self.from_y,
                end_y: self.to_y,
            },
            Edge {
                start_x: self.to_x,
                end_x: self.to_x,
                start_y: self.from_y,
                end_y: self.to_y,
            },
            Edge {
                start_x: self.from_x,
                end_x: self.to_x,
                start_y: self.from_y,
                end_y: self.from_y,
            },
            Edge {
                start_x: self.from_x,
                end_x: self.to_x,
                start_y: self.to_y,
                end_y: self.to_y,
            },
        ]
    }

    fn is_inside_polygon(&self, edges: &Vec<Edge>) -> bool {
        // to be fully inside a polygon, a rectangle must:
        // have all corners inside
        is_inside_polygon(self.from_x, self.from_y, edges)
            && is_inside_polygon(self.from_x, self.to_y, edges)
            && is_inside_polygon(self.to_x, self.from_y, edges)
            && is_inside_polygon(self.to_x, self.to_y, edges)
            // have none of its edges intersect the polygon's edges
            && !self.get_edges().iter().any(|rectangle_edge| {
                edges
                    .iter()
                    .any(|polygon_edge| polygon_edge.intersects(&rectangle_edge))
            })
    }
}

impl Day for Day09 {
    fn part_one(input: &str) -> impl Display {
        let tiles = parse_input(input);

        tiles
            .iter()
            .combinations(2)
            .map(|corners| rectangle_area(corners[0], corners[1]))
            .max()
            .unwrap()
    }

    fn part_two(input: &str) -> impl Display {
        let (tiles, edges) = parse_input_part_two(input);

        tiles
            .iter()
            .combinations(2)
            .map(|corners| Rectangle::new(corners[0], corners[1]))
            .sorted_by_key(|rectangle| rectangle.size)
            .rev()
            .find(|rectangle| rectangle.is_inside_polygon(&edges))
            .unwrap()
            .size
    }

    fn get_day_num() -> u8 {
        return 9;
    }
}

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(",").map(|coord_str| coord_str.parse().unwrap());
            Tile {
                x: coords.next().unwrap(),
                y: coords.next().unwrap(),
            }
        })
        .collect()
}

fn parse_input_part_two(input: &str) -> (Vec<Tile>, Vec<Edge>) {
    let tiles = parse_input(input);
    let mut edges: Vec<Edge> = tiles
        .iter()
        .tuple_windows()
        .map(|(&start, &end)| Edge::new(start, end))
        .collect();

    // list wraps around, must add edge from last tile to first tile
    edges.push(Edge::new(tiles[tiles.len() - 1], tiles[0]));

    (tiles, edges)
}

fn rectangle_area(tile_a: &Tile, tile_b: &Tile) -> isize {
    (max(tile_a.x, tile_b.x) + 1 - min(tile_a.x, tile_b.x))
        * (max(tile_a.y, tile_b.y) + 1 - min(tile_a.y, tile_b.y))
}

fn is_inside_polygon(x: f64, y: f64, edges: &Vec<Edge>) -> bool {
    ({ edges.iter().any(|edge| edge.is_point_on_edge(x, y)) })
        || ({
            edges
                .iter()
                .filter(|edge| edge.ray_intersects_edge(x, y))
                .count()
                % 2
                == 1
        })
}
