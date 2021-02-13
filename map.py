import curses
from main import panes



world_map = (
            (^^^^^^^^^^),
            (^^^^^^^^^^),
            (.....^^^^^),
            (........^^),
            (,,........),
            (~~~~......),
            (~~~~~~..K.),
            (~~~~~~~~~.),
            (~~~~~...~~)

)


world_map_chars = {}


def draw_map():
    for row in range(len(world_map)):
        for col in range(len(world_map[row])):
            artscr.addch(row, col, world_map_chars[world_map[row]]
