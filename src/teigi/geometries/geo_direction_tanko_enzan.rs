#![allow(dead_code)]
//!
//! 単項演算☆（＾～＾）
//!

use super::super::super::teigi::geometries::geo_teigi::Point;

impl Point {
    /**
     * 西 p
     */
    pub fn to_east(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    /**
     * 北西 p
     */
    pub fn to_north_east(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    /**
     * 北北西 p
     */
    pub fn to_north_north_east(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 2,
        }
    }

    /**
     * 北 p
     */
    pub fn to_north(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    /**
     * 北北西 p
     */
    pub fn to_north_north_west(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y + 2,
        }
    }

    /**
     * 北西 p
     */
    pub fn to_north_west(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    /**
     * 西 p
     */
    pub fn to_west(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    /**
     * 南西 p
     */
    pub fn to_south_west(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    /**
     * 南南西 p
     */
    pub fn to_south_south_west(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y - 2,
        }
    }

    /**
     * 南 p
     */
    pub fn to_south(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    /**
     * 南南東 p
     */
    pub fn to_south_south_east(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y - 2,
        }
    }

    /**
     * 南東 p
     */
    pub fn to_south_east(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
}
