pub enum Direction {
    Left,
    Right,
}

impl Direction {
    /// Applies the direction to a `usize` value, wrapping it around a given limit.
    ///
    /// This method moves the value `i` one step in the specified direction. The
    /// operation is circular, meaning that moving left from `0` results in `limit - 1`,
    /// and moving right from `limit - 1` results in `0`. This is achieved using
    /// the `rem_euclid` method to handle wrapping correctly for both directions.
    pub fn apply_usize(&self, i: &mut usize, limit: usize) {
        *i = match self {
            Direction::Left => {
                if *i == 0 {
                    limit-1
                } else {
                    *i - 1
                }
            }
            Direction::Right => {
                if *i == limit-1 {
                    0
                } else {
                    *i + 1
                }
            }
        }
        .rem_euclid(limit);
    }
}