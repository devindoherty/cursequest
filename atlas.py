import curses
from main import screens

world_map = (
        (3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3),
        (3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3),
        (1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3),
        (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3),
        (2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3),
        (2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3),
        (2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 3, 3, 3, 3),
        (2, 2, 2, 2, 2, 2, 2, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3),
        (2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2),
        (2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2),
        (2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 1, 1, 1),
        (1, 2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 1, 1, 1, 1, 1),
        (1, 1, 2, 2, 2, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 1, 1, 1, 1, 1),
        (1, 1, 1, 2, 2, 2, 4, 4, 4, 4, 4, 2, 2, 2, 2, 1, 1, 1, 1, 1),
        (1, 1, 1, 2, 2, 2, 2, 2, 4, 4, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1),
        (1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1),
        (5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 2, 2, 2, 2, 1, 1, 1, 1, 1),
        (5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 2, 2, 2, 2, 1, 1, 1, 1),
        (5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1),
        (5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5)
        )

#map_legend = {
#        0: [" ", "empty"],
#        1: [".", "plain"],
#        2: [",", "grassland"],
#        3: ["A", "mountain"],
#        4: ["~", "water"],
#        5: ["t", "forest"],
#        6: ["K", "keep"],
#        7: ["C", "city"],
#        8: ["H", "hall"],
#        9: ["E", "enclave"]
#        }

map_legend = {0:" ", 1:".", 2:",", 3:"A", 4:"~", 5:"t", 6:"K", 7:"C", 8:"H", 9:"E", 10:"@"}



def draw_map(artscr, world_map):
    for row in range(len(world_map)):
        for col in range(len(world_map[row])):
            artscr.addch(row, col, map_legend[world_map[row][col]])
        
def draw_player_position(artscr, world_map):
    player_y = 11
    player_x = 11
    while True:
        key = curses.artscr.getch()
        
        if key == curses.KEY_UP:
            player_y -= 1
        
        elif key == curses.KEY_DOWN:
            player_y += 1
        
        elif key == curses.KEY_LEFT:
            player_x -= 1
        
        elif key == curses.KEY_RIGHT:
            player_x += 1
        
        draw_map(artscr, world_map)
        artscr.addch(player_y, player_x, "@")
        artscr.refresh()
        
        
        
