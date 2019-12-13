use std::borrow::BorrowMut;
use std::cmp::min;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::str::FromStr;

type Grid2 = Vec<Vec<Vec<i32>>>;

struct Grid {
    points: Vec<(i32, i32)>
}

struct Direction {
    orientation: char,
    amount: i32,
}

struct Segment {
    start: (i32, i32),
    end: (i32, i32),
}

fn contains_point(grid: &Vec<(i32, i32)>, point: (i32, i32)) -> bool {
    return grid.contains(&point);
}

fn add_point_to_grid(grid: &mut Grid, point: (i32, i32)) {
    grid.points.push(point);
}


fn main() -> io::Result<()> {
    let file = File::open("src/day03.txt")?;
    let mut reader = BufReader::new(file);
    let mut lines = reader.by_ref().lines();
    let mut grid: Grid = Grid { points: vec![] };
    let mut min_distance = (0, 0, std::i32::MAX);
    let mut lines_segments: Vec<Vec<Segment>> = vec![];
    for line in lines {
        let line2 = line.unwrap().clone();
        let line_vec: Vec<&str> = line2.split(",").collect();
        let mut distances_vec: Vec<Direction> = line_vec.into_iter().map(|entry| build_direction(entry[0..1].as_ref(), i32::from_str(&entry[1..]).unwrap_or(-9999))).collect();

        let mut segments: Vec<Segment> = vec![];
        lines_segments.push(draw_line(grid.borrow_mut(), distances_vec, &mut min_distance));
    }
    assert_eq!(lines_segments.len(), 2);
    let mut i = 0;
    for line_segment in &lines_segments[0] {
        let mut min_dist = std::i32::MAX;
        let mut j = 0;
        for line2_segment in &lines_segments[1] {
            let intersect_result = does_intersect(line_segment.start, line_segment.end, line2_segment.start, line2_segment.end);
            if intersect_result.0 {
                // manhattan
                let mut x1 = 0;
                let mut y1 = 0;
                let mut x2 = (intersect_result.1).0;
                let mut y2 = (intersect_result.1).1;
                let distance = (x1 - x2).abs() + (y1 - y2).abs();
                //check if smallest
                if (distance < min_dist) {
                    min_dist = distance;
                    println!("found smaller value {}, at {},{}", min_dist, x2, y2);
                    println!("lines.len {}|{}, start_index i,j {},{}",lines_segments[0].len(),lines_segments[1].len(),i,j);
                    println!("combined distance {}",walk_back(&lines_segments[0],i,(x2,y2))+walk_back(&lines_segments[1],j,(x2,y2)));
                }
            }
            j+=1;
        }
        i+=1;
    }

    Ok(())
}

fn walk_back(lines: &Vec<Segment>, start_index: i32, start_point: (i32, i32)) -> i32 {
    let mut amount_walked: i32 = 0;
    assert!(start_index<(lines.len() as i32));
    //calc distance to start of last segment (as one coord is the same we could just determine which one - but we use the universal solution
    let dist_to_last_seg = manhattan_distance_between(lines[start_index as usize].start,  start_point);
    amount_walked += dist_to_last_seg;

    for i in lines[..start_index as usize].iter().rev()
    {
      amount_walked += manhattan_distance_between(i.start,i.end);
    }
    
    return amount_walked;
}

fn euclidian_distance_between(point1: (i32, i32), point2: (i32, i32)) -> i32 {
    return (manhattan_distance_between(point1,point2) as f32).sqrt() as i32;
}
fn manhattan_distance_between(point1: (i32, i32), point2: (i32, i32)) -> i32 {
    return ((point1.0 - point2.0).abs() + (point1.1 - point2.1).abs()) as i32;
}


fn build_direction(ori: &str, dist: i32) -> Direction {
    return Direction {
        orientation: ori.parse().unwrap(),
        amount: dist,
    };
}

//this is a bit messy - i opted for a different approach but did not clean up the unnecessary bits
//min distance: x,y,distance
fn draw_line(grid: &mut Grid, line: Vec<Direction>, min_distance: &mut (i32, i32, i32)) -> Vec<Segment> {
    //horizontal, vertical, num_wires
    let mut position = (0, 0);
    let mut lines_segments = vec![];
    for direction in line {
        let coords = match direction.orientation {
            'U' => (position.0, position.1 - direction.amount),
            'D' => (position.0, position.1 + direction.amount),
            'R' => (position.0 + direction.amount, position.1),
            'L' => (position.0 - direction.amount, position.1),
            _ => (0, 0),
        };
        lines_segments.push(Segment { start: position, end: coords });

        position = coords;
        add_point_to_grid(grid, coords);
//        println!("points in grid: {}", grid.points.len());
        //println!("{},{}", direction.orientation, direction.amount);
        //check if crosses
        if (contains_point(grid.points.as_ref(), coords)) {
            //calc manhattan distance
            let mut x1 = 0;
            let mut y1 = 0;
            let mut x2 = coords.0;
            let mut y2 = coords.1;
            let distance = (x1 - x2).abs() + (y1 - y2).abs();
            //check if smallest
            if (distance < min_distance.2) {
                min_distance.0 = coords.0;
                min_distance.1 = coords.1;
                min_distance.2 = distance;
                //   println!("found smaller value {}, at {},{}", min_distance.2, min_distance.0, min_distance.1);
            }
            //println!("distance value {}, at {},{}", min_distance.2, min_distance.0, min_distance.1);
        }
    }
    return lines_segments;
}

//start and end point of each line
fn does_intersect(mut start_1: (i32, i32), mut end_1: (i32, i32), mut start_2: (i32, i32), mut end_2: (i32, i32)) -> (bool, (i32, i32)) {
    let mut la1: (i32, i32);//Line 1 Anfang
    let mut le1: (i32, i32);//Line 1 Ende
    let mut la2: (i32, i32);//Line 2 Anfang
    let mut le2: (i32, i32);//Line 2 Ende

    //determine orientation, and swap x and y axis if our line is horiz
    if (start_1.1 == end_1.1) {
        //swap x and y
        start_1 = (start_1.1, start_1.0);
        start_2 = (start_2.1, start_2.0);
        end_1 = (end_1.1, end_1.0);
        end_2 = (end_2.1, end_2.0);
    }

    if (start_1.1 > end_1.1) {
        la1 = start_1;
        le1 = end_1;
    } else {
        la1 = end_1;
        le1 = start_1;
    }
    if (start_2.0 < end_2.0) {
        la2 = start_2;
        le2 = end_2;
    } else {
        la2 = end_2;
        le2 = start_2;
    }
    let between_vert = la2.1 <= la1.1 && la2.1 >= le1.1;
    let between_hori = la2.0 <= la1.0 && le2.0 >= la1.0;
    let intersection_point = (la1.0, la2.1);
    return (between_vert && between_hori, intersection_point);
}

#[cfg(test)]
mod tests {
    use crate::{contains_point, does_intersect, Grid,  manhattan_distance_between};

    #[test]
    fn test_contains_point() {
        let grid: Grid = Grid {
            points: vec![(1, 1), (2, 2)]
        };
        assert_eq!(contains_point(grid.points.as_ref(), (0, 0)), false);
        assert_eq!(contains_point(grid.points.as_ref(), (2, 2)), true);
    }

    #[test]
    fn test_intersect_vert() {
        assert_eq!(does_intersect((7, 3), (7, 10), (6, 6), (12, 6)).0, true);
        assert_ne!(does_intersect((13, 3), (13, 10), (6, 6), (12, 6)).0, true);

        assert_eq!(does_intersect((7, 3), (7, 10), (6, 6), (12, 6)).1, (7, 6));
    }

    #[test]
    fn test_intersect_hori() {
        assert_eq!(does_intersect((6, 6), (12, 6), (7, 3), (7, 10)).0, true);
        assert_eq!(does_intersect((6, 6), (12, 6), (7, 3), (7, 10)).1, (6, 7));
    }

    #[test]
    fn test_dist_between() {
        assert_eq!(manhattan_distance_between((6, 6), (12, 6)) ,6);
    }
}
