type Position = (usize, usize);

struct MineSweeper {
    width: usize,
    height: usize,
    open_tiles: Vec<Position>,
    mines: Vec<Position>,
    flagged_tiles: Vec<Position>,
}
