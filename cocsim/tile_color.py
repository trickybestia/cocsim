from pygame.color import Color

DROP_ZONE_TILE_EVEN_COLOR = Color(50, 70, 40)
DROP_ZONE_TILE_ODD_COLOR = Color(30, 30, 10)
DROP_ZONE_BORDER_TILE_EVEN_COLOR = Color(50, 70, 0)
DROP_ZONE_BORDER_TILE_ODD_COLOR = Color(30, 30, 0)

TILE_EVEN_COLOR = Color(40, 40, 40)
TILE_ODD_COLOR = Color(10, 10, 10)
BORDER_TILE_EVEN_COLOR = Color(30, 60, 0)
BORDER_TILE_ODD_COLOR = Color(20, 30, 0)


def get_tile_color(even: bool, border: bool, drop_zone: bool) -> Color:
    if drop_zone:
        if border:
            return (
                DROP_ZONE_BORDER_TILE_EVEN_COLOR
                if even
                else DROP_ZONE_BORDER_TILE_ODD_COLOR
            )
        else:
            return (
                DROP_ZONE_TILE_EVEN_COLOR if even else DROP_ZONE_TILE_ODD_COLOR
            )
    else:
        if border:
            return BORDER_TILE_EVEN_COLOR if even else BORDER_TILE_ODD_COLOR
        else:
            return TILE_EVEN_COLOR if even else TILE_ODD_COLOR
