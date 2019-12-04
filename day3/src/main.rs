use common::grid;

fn main() -> Result<(), String> {
    let input = include_str!("input/input1");

    let paths = input
        .lines()
        .map(|line| line.parse::<grid::Path>())
        .collect::<Result<Vec<_>, _>>()?;

    let p0 = &paths[0];
    let p1 = &paths[1];

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

    println!(
        "Part2 = {}",
        intersection_points
            .iter()
            .min()
            .unwrap()
            .steps
    );

    Ok(())
}
