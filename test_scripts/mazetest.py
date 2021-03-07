import curses
import random
import time

g_map = (
        (1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
        (1, 0, 0, 0, 0, 0, 0, 0, 0, 1),
        (1, 0, 0, 0, 0, 0, 0, 0, 0, 1),
        (1, 0, 0, 0, 0, 0, 0, 0, 0, 1),
        (1, 0, 0, 0, 0, 0, 0, 0, 0, 1),
        (1, 0, 0, 0, 0, 0, 0, 0, 0, 1),
        (1, 0, 0, 0, 0, 0, 0, 0, 0, 1),
        (1, 0, 0, 0, 0, 0, 0, 0, 0, 1),
        (1, 0, 0, 0, 0, 0, 0, 0, 0, 1),
        (1, 1, 1, 1, 1, 1, 1, 1, 1, 1)
)

g_tileCharsMap = {0:' ', 1:'H'}

def drawMap(_screen, _map):
    for row in range(len(_map)):
        for col in range(len(_map[row])):
            _screen.addch(row, col, g_tileCharsMap[_map[row][col]])

stdscr = curses.initscr()
stdscr.nodelay(1) #cursor.getch won't wait for input
curses.curs_set(0) #hide cursor

drawMap(stdscr, g_map)

curses.endwin()
stdscr.getch()
