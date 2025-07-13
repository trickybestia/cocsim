use crate::consts::*;

pub fn get_tile_color(even: bool, border: bool, drop_zone: bool, occupied: bool) -> &'static str {
    if occupied {
        if even {
            BUILDING_TILE_EVEN_COLOR
        } else {
            BUILDING_TILE_ODD_COLOR
        }
    } else if drop_zone {
        if border {
            if even {
                DROP_ZONE_BORDER_TILE_EVEN_COLOR
            } else {
                DROP_ZONE_BORDER_TILE_ODD_COLOR
            }
        } else {
            if even {
                DROP_ZONE_TILE_EVEN_COLOR
            } else {
                DROP_ZONE_TILE_ODD_COLOR
            }
        }
    } else {
        if border {
            if even {
                BORDER_TILE_EVEN_COLOR
            } else {
                BORDER_TILE_ODD_COLOR
            }
        } else {
            if even {
                TILE_EVEN_COLOR
            } else {
                TILE_ODD_COLOR
            }
        }
    }
}
