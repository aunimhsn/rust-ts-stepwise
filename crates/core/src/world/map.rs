//! The map of the world: its cells, their paints and items, and the painted passages.

use crate::color::Color;

/// The most columns a map has.
pub const MAX_COLUMNS: u8 = 16;
/// The most rows a map has.
pub const MAX_ROWS: u8 = 12;

/// The place of a cell: `x` the column from the left, `y` the row from the top, both from 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    /// The column, from 0 on the left.
    pub x: u8,
    /// The row, from 0 at the top.
    pub y: u8,
}

impl Point {
    /// The cell at column `x` and row `y`.
    #[must_use]
    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    /// The next cell in `direction`, or `None` past the top or the left of any map (the
    /// right and the bottom are the map's to check).
    #[must_use]
    pub fn toward(self, direction: Direction) -> Option<Self> {
        let Self { x, y } = self;
        Some(match direction {
            Direction::North => Self::new(x, y.checked_sub(1)?),
            Direction::East => Self::new(x.checked_add(1)?, y),
            Direction::South => Self::new(x, y.checked_add(1)?),
            Direction::West => Self::new(x.checked_sub(1)?, y),
        })
    }

    /// The direction from this cell to `other`, when `other` is next to it (not diagonally).
    #[must_use]
    pub fn direction_to(self, other: Self) -> Option<Direction> {
        Direction::ALL
            .into_iter()
            .find(|&direction| self.toward(direction) == Some(other))
    }
}

/// One of the four directions, on the map: north is up, toward the row 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Up.
    North,
    /// Right.
    East,
    /// Down.
    South,
    /// Left.
    West,
}

impl Direction {
    /// The four directions, clockwise from the north.
    pub const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    /// The direction a quarter turn to the left.
    #[must_use]
    pub const fn turned_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    /// The direction a quarter turn to the right.
    #[must_use]
    pub const fn turned_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

/// A cell of the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// A wall: nothing goes in it.
    Wall,
    /// A floor cell, with what lies on it.
    Floor(Floor),
}

/// What lies on a floor cell. Its passages east and south are kept here; the ones north and
/// west are the passages south and east of its neighbors, so that each is kept once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Floor {
    paint: Option<Color>,
    item: bool,
    east: bool,
    south: bool,
}

impl Floor {
    /// The paint of the cell.
    #[must_use]
    pub const fn paint(self) -> Option<Color> {
        self.paint
    }

    /// Whether an item lies on the cell.
    #[must_use]
    pub const fn has_item(self) -> bool {
        self.item
    }
}

/// Why a map or a world can't be built as asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
    /// A map without cells, or larger than [`MAX_COLUMNS`] × [`MAX_ROWS`].
    Size {
        /// The columns asked.
        width: usize,
        /// The rows asked.
        height: usize,
    },
    /// A cell outside the map.
    OutsideMap(Point),
    /// A paint, an item, a passage or the robot on a wall.
    OnWall(Point),
    /// A wall on a cell that has a paint or an item: the walls are set first.
    NotEmpty(Point),
    /// A passage between two cells that aren't next to each other.
    NotNeighbors(Point, Point),
    /// A passage painted from a cell that has no paint: a painted passage always joins two
    /// painted cells.
    Unpainted(Point),
}

/// A grid of cells, at most [`MAX_COLUMNS`] × [`MAX_ROWS`]; outside it counts as a wall.
///
/// A painted passage always joins two painted floor cells: painting a passage needs both
/// paints, and erasing a cell, or turning it into a wall, removes its passages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    width: u8,
    height: u8,
    /// The cells row by row, from the top left.
    cells: Vec<Cell>,
}

impl Map {
    /// A map of `width` columns and `height` rows, all floor cells without paint nor item.
    ///
    /// # Errors
    ///
    /// [`BuildError::Size`] when the map has no cell or is larger than the most allowed.
    pub fn new(width: usize, height: usize) -> Result<Self, BuildError> {
        let size = BuildError::Size { width, height };
        let (Ok(columns), Ok(rows)) = (u8::try_from(width), u8::try_from(height)) else {
            return Err(size);
        };
        if !(1..=MAX_COLUMNS).contains(&columns) || !(1..=MAX_ROWS).contains(&rows) {
            return Err(size);
        }
        let (width, height) = (columns, rows);
        let count = usize::from(width).saturating_mul(usize::from(height));
        let cells = vec![Cell::Floor(Floor::default()); count];
        Ok(Self {
            width,
            height,
            cells,
        })
    }

    /// The number of columns.
    #[must_use]
    pub const fn width(&self) -> u8 {
        self.width
    }

    /// The number of rows.
    #[must_use]
    pub const fn height(&self) -> u8 {
        self.height
    }

    /// Every cell of the map, row by row from the top left.
    pub fn points(&self) -> impl Iterator<Item = Point> {
        let width = self.width;
        (0..self.height).flat_map(move |y| (0..width).map(move |x| Point::new(x, y)))
    }

    /// The cell at `point`, or `None` outside the map.
    #[must_use]
    pub fn cell(&self, point: Point) -> Option<Cell> {
        self.cells.get(self.index(point)?).copied()
    }

    /// What lies on the cell at `point`, or `None` for a wall or outside the map.
    #[must_use]
    pub fn floor(&self, point: Point) -> Option<Floor> {
        match self.cell(point)? {
            Cell::Floor(floor) => Some(floor),
            Cell::Wall => None,
        }
    }

    /// Whether the cell at `point` is a floor cell of the map.
    #[must_use]
    pub fn is_floor(&self, point: Point) -> bool {
        self.floor(point).is_some()
    }

    /// The paint of the cell at `point`, `None` for a wall or outside the map.
    #[must_use]
    pub fn paint(&self, point: Point) -> Option<Color> {
        self.floor(point)?.paint
    }

    /// Whether an item lies on the cell at `point`.
    #[must_use]
    pub fn has_item(&self, point: Point) -> bool {
        self.floor(point).is_some_and(|floor| floor.item)
    }

    /// Whether the passage from the cell at `point` to its neighbor in `direction` is painted.
    #[must_use]
    pub fn passage(&self, point: Point, direction: Direction) -> bool {
        let Some((point, eastward)) = Self::passage_key(point, direction) else {
            return false;
        };
        self.floor(point)
            .is_some_and(|floor| if eastward { floor.east } else { floor.south })
    }

    /// Turns the cell at `point` into a wall (nothing changes when it is one already).
    ///
    /// # Errors
    ///
    /// [`BuildError::OutsideMap`] when `point` is outside the map, [`BuildError::NotEmpty`]
    /// when its cell has a paint or an item: the walls are set first, so that a level never
    /// loses one in silence. An unpainted cell has no painted passage.
    pub fn set_wall(&mut self, point: Point) -> Result<(), BuildError> {
        let cell = self.cell_mut(point).ok_or(BuildError::OutsideMap(point))?;
        if let Cell::Floor(floor) = cell
            && (floor.paint.is_some() || floor.item)
        {
            return Err(BuildError::NotEmpty(point));
        }
        *cell = Cell::Wall;
        Ok(())
    }

    /// Paints the cell at `point` with `color`, over any paint already there.
    ///
    /// # Errors
    ///
    /// When `point` is outside the map or a wall.
    pub fn set_paint(&mut self, point: Point, color: Color) -> Result<(), BuildError> {
        self.floor_mut(point)?.paint = Some(color);
        Ok(())
    }

    /// Puts an item on the cell at `point` (nothing changes when one is already there).
    ///
    /// # Errors
    ///
    /// When `point` is outside the map or a wall.
    pub fn set_item(&mut self, point: Point) -> Result<(), BuildError> {
        self.floor_mut(point)?.item = true;
        Ok(())
    }

    /// Paints the passage between the cells at `from` and `to`, which must be next to each
    /// other and both painted.
    ///
    /// # Errors
    ///
    /// When the two cells aren't next to each other, or one is outside the map, a wall or
    /// unpainted.
    pub fn set_passage(&mut self, from: Point, to: Point) -> Result<(), BuildError> {
        let direction = from
            .direction_to(to)
            .ok_or(BuildError::NotNeighbors(from, to))?;
        for point in [from, to] {
            let floor = match self.cell(point) {
                Some(Cell::Floor(floor)) => floor,
                Some(Cell::Wall) => return Err(BuildError::OnWall(point)),
                None => return Err(BuildError::OutsideMap(point)),
            };
            if floor.paint.is_none() {
                return Err(BuildError::Unpainted(point));
            }
        }
        self.set_passage_paint(from, direction, true);
        Ok(())
    }

    /// Removes the paint of the cell at `point` and of its four passages.
    pub(super) fn erase(&mut self, point: Point) {
        if let Ok(floor) = self.floor_mut(point) {
            floor.paint = None;
        }
        for direction in Direction::ALL {
            self.set_passage_paint(point, direction, false);
        }
    }

    /// Paints the cell at `point` with `color`; nothing happens off the floor.
    pub(super) fn paint_cell(&mut self, point: Point, color: Color) {
        if let Ok(floor) = self.floor_mut(point) {
            floor.paint = Some(color);
        }
    }

    /// Paints `to` with `color` and the passage from `from`, which the robot just crossed with
    /// its brush down. The passage is painted only when `from` is painted, as it always is with
    /// the brush down, so that the map keeps its rule on its own.
    pub(super) fn paint_step(&mut self, from: Point, to: Point, color: Color) {
        self.paint_cell(to, color);
        if self.paint(from).is_none() || self.paint(to).is_none() {
            return;
        }
        if let Some(direction) = from.direction_to(to) {
            self.set_passage_paint(from, direction, true);
        }
    }

    /// Puts or removes the item of the cell at `point`; nothing happens off the floor.
    pub(super) fn put_item(&mut self, point: Point, item: bool) {
        if let Ok(floor) = self.floor_mut(point) {
            floor.item = item;
        }
    }

    fn set_passage_paint(&mut self, point: Point, direction: Direction, painted: bool) {
        let Some((point, eastward)) = Self::passage_key(point, direction) else {
            return;
        };
        if let Ok(floor) = self.floor_mut(point) {
            if eastward {
                floor.east = painted;
            } else {
                floor.south = painted;
            }
        }
    }

    /// The cell that keeps the passage from `point` in `direction`, and whether it is its
    /// passage east (else south); `None` past the top or the left.
    fn passage_key(point: Point, direction: Direction) -> Option<(Point, bool)> {
        match direction {
            Direction::East => Some((point, true)),
            Direction::South => Some((point, false)),
            Direction::West => Some((point.toward(Direction::West)?, true)),
            Direction::North => Some((point.toward(Direction::North)?, false)),
        }
    }

    fn index(&self, point: Point) -> Option<usize> {
        if point.x >= self.width || point.y >= self.height {
            return None;
        }
        usize::from(point.y)
            .checked_mul(usize::from(self.width))?
            .checked_add(usize::from(point.x))
    }

    fn cell_mut(&mut self, point: Point) -> Option<&mut Cell> {
        let index = self.index(point)?;
        self.cells.get_mut(index)
    }

    fn floor_mut(&mut self, point: Point) -> Result<&mut Floor, BuildError> {
        match self.cell_mut(point) {
            Some(Cell::Floor(floor)) => Ok(floor),
            Some(Cell::Wall) => Err(BuildError::OnWall(point)),
            None => Err(BuildError::OutsideMap(point)),
        }
    }
}
