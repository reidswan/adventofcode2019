use common::grid;

pub fn part1(paths: &(grid::Path, grid::Path)) {
    let (p0, p1) = paths;
    let intersection_points = p0
        .line_segments
        .iter()
        .flat_map(|seg0| {
            p1.line_segments
                .iter()
                .filter_map(|seg1| {
                    let coord = seg0.intersection_point(seg1)?;
                    let steps0 = seg0.with_steps_at(coord)?;
                    let steps1 = seg1.with_steps_at(coord)?;
                    Some(steps0 + steps1)
                })
                .collect::<Vec<grid::SteppedCoordinate>>()
        })
        .collect::<Vec<grid::SteppedCoordinate>>();

    println!(
        "Part1 = {}",
        intersection_points
            .iter()
            .map(|a| a.coordinate)
            .min()
            .unwrap()
            .manhattan_distance()
    );
}

pub fn part2(paths: &(grid::Path, grid::Path)) {
    let (p0, p1) = paths;
    let intersection_points = p0
        .line_segments
        .iter()
        .flat_map(|seg0| {
            p1.line_segments
                .iter()
                .filter_map(|seg1| {
                    let coord = seg0.intersection_point(seg1)?;
                    let steps0 = seg0.with_steps_at(coord)?;
                    let steps1 = seg1.with_steps_at(coord)?;
                    Some(steps0 + steps1)
                })
                .collect::<Vec<grid::SteppedCoordinate>>()
        })
        .collect::<Vec<grid::SteppedCoordinate>>();

    println!(
        "Part2 = {}",
        intersection_points
            .iter()
            .min()
            .unwrap()
            .steps
    );
    
}

pub fn get_parsed_input() -> (grid::Path, grid::Path) {
    let mut path_vec = include_str!("input/input1")
        .lines()
        .map(|line| line.parse::<grid::Path>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let p1 = path_vec.pop().unwrap();
    let p2 = path_vec.pop().unwrap();

    (p1, p2)
}
