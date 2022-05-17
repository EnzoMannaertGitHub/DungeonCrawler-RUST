use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub botom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        Self {
            left_x: player_position.x - DISPLAY_WIDTH / 2,
            right_x: player_position.x + DISPLAY_WIDTH / 2,
            top_y: player_position.y - DISPLAY_HEIGHT / 2,
            botom_y: player_position.y + DISPLAY_HEIGHT / 2,
        }
    }

    pub fn on_player_move(&mut self, player_position: Point, map: &Map) {
        let left_x = player_position.x - DISPLAY_WIDTH / 2;
        let right_x = player_position.x + DISPLAY_WIDTH / 2;
        let top_y = player_position.y - DISPLAY_HEIGHT / 2;
        let botom_y = player_position.y + DISPLAY_HEIGHT / 2;

        let is_x_in_bounds = map.in_bounds(Point::new(left_x, SCREEN_HEIGHT / 2))
            && map.in_bounds(Point::new(right_x, SCREEN_HEIGHT / 2));

        let is_y_in_bounds = map.in_bounds(Point::new(SCREEN_WIDTH / 2, top_y))
            && map.in_bounds(Point::new(SCREEN_WIDTH / 2, botom_y));

        if is_x_in_bounds {
            self.left_x = left_x;
            self.right_x = right_x;
        }

        if is_y_in_bounds {
            self.top_y = top_y;
            self.botom_y = botom_y;
        }
    }
}
