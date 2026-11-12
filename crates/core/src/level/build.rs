//! What a level file describes, built into worlds and goals: the walls first, then the items,
//! the paints and the robot, with the methods of the map, which refuse what can't be; then
//! the goals, which check their own rules.

use super::file::{GoalsFile, LevelFile, MapFile, SandboxFile, StrokeFile, point};
use super::{
    FLOOR, Goals, Level, LevelError, LevelErrorKind, LevelMap, MAX_MAPS, Sandbox, Stroke, WALL,
};
use crate::world::{Map, Point, Robot, World};

/// A level, its maps numbered from 1 in its errors.
pub(super) fn level(file: LevelFile) -> Result<Level, LevelError> {
    let count = file.maps.len();
    if !(1..=MAX_MAPS).contains(&count) {
        return Err(LevelError::file(LevelErrorKind::MapCount(count)));
    }
    let maps = (1..)
        .zip(file.maps)
        .map(|(number, map)| level_map(map).map_err(|kind| LevelError::map(number, kind)))
        .collect::<Result<_, _>>()?;
    Ok(Level {
        starting_code: file.starting_code,
        maps,
    })
}

/// The sandbox, whose map has no goals.
pub(super) fn sandbox(file: SandboxFile) -> Result<Sandbox, LevelError> {
    if file.map.goals.is_some() {
        return Err(LevelError::file(LevelErrorKind::GoalsInSandbox));
    }
    let world = world(&file.map).map_err(LevelError::file)?;
    Ok(Sandbox {
        starting_code: file.starting_code,
        world,
    })
}

fn level_map(map: MapFile) -> Result<LevelMap, LevelErrorKind> {
    let world = world(&map)?;
    let goals = goals(map.goals.ok_or(LevelErrorKind::NoGoal)?, &world)?;
    Ok(LevelMap { world, goals })
}

/// The world of a map as it starts.
fn world(map: &MapFile) -> Result<World, LevelErrorKind> {
    let mut grid = terrain(&map.terrain)?;
    for &at in &map.items {
        let at = point(at);
        if grid.has_item(at) {
            return Err(LevelErrorKind::DuplicateItem(at));
        }
        grid.set_item(at).map_err(LevelErrorKind::Map)?;
    }
    for stroke in &map.paints {
        paint(&mut grid, &self::stroke(stroke)?)?;
    }
    let robot = Robot::new(point(map.robot.at), map.robot.facing.into());
    let robot = if map.robot.carrying {
        robot.carrying()
    } else {
        robot
    };
    World::new(grid, robot).map_err(LevelErrorKind::Map)
}

/// The map drawn by the rows of the terrain: [`WALL`] a wall, [`FLOOR`] a floor cell.
fn terrain(terrain: &str) -> Result<Map, LevelErrorKind> {
    let rows: Vec<&str> = terrain.lines().collect();
    let width = rows.first().map_or(0, |row| row.chars().count());
    let mut map = Map::new(width, rows.len()).map_err(LevelErrorKind::Map)?;
    for (y, row) in (0..map.height()).zip(&rows) {
        let length = row.chars().count();
        if length != width {
            return Err(LevelErrorKind::RowLength {
                y,
                length,
                expected: width,
            });
        }
        for (x, character) in (0..map.width()).zip(row.chars()) {
            let at = Point::new(x, y);
            match character {
                WALL => map.set_wall(at).map_err(LevelErrorKind::Map)?,
                FLOOR => {}
                character => return Err(LevelErrorKind::UnknownCharacter { at, character }),
            }
        }
    }
    Ok(map)
}

/// Paints `stroke` on `map`, its cells and its passages, over any paint already there.
fn paint(map: &mut Map, stroke: &Stroke) -> Result<(), LevelErrorKind> {
    for &at in stroke.path() {
        map.set_paint(at, stroke.color())
            .map_err(LevelErrorKind::Map)?;
    }
    for (from, to) in stroke.steps() {
        map.set_passage(from, to).map_err(LevelErrorKind::Map)?;
    }
    Ok(())
}

fn stroke(file: &StrokeFile) -> Result<Stroke, LevelErrorKind> {
    Stroke::new(file.color, file.path.iter().copied().map(point).collect())
}

fn goals(file: GoalsFile, world: &World) -> Result<Goals, LevelErrorKind> {
    let paints = file.paints.iter().map(stroke).collect::<Result<_, _>>()?;
    Goals::new(
        file.robot_at.map(point),
        paints,
        file.nothing_else_painted,
        file.items_at.into_iter().map(point).collect(),
        file.carrying,
        world,
    )
}
