import curses
import time
import os
import sys


# Setting up two windows, one for ASCII art and maps and whatnot, the other for text output/input

def panes(stdscr):
    
    # Blank the canvas
    stdscr.clear()
   
    # Border for entire terminal
    #stdscr.box()

    # Draw all of that
    stdscr.refresh()
    

    
    # Finding Terminal size. Height = y, width = x
    height, width = stdscr.getmaxyx()
    
    # Finding starting y,x for art and text panes
    art_x = 0
    art_y = 0
    text_x = 0 
    text_y = int((height // 2) + 1)

    # Setup the Art Pane with artscr
    artscr = curses.newwin(int(height // 2) - 1, width, art_y, art_x)
    artscr.addstr(1, 1, "This is the Art Window")
    artscr.box()
    
    # Color Testing
    curses.start_color()
    curses.use_default_colors()
    for i in range(0, curses.COLORS):
        curses.init_pair(i + 1, i, -1)
    try:
        for i in range(0, 255):
           artscr.addstr(str(i), curses.color_pair(i))
    except curses.ERR:
        pass


    # Setup the Text Pane with textscr
    textscr = curses.newwin(int(height // 2) , width, text_y, text_x)
    textscr.addstr(1, 1, "This is the Text Window")  
    textscr.box()

    # Print our different screens
    artscr.refresh()
    textscr.refresh()
    stdscr.refresh()
    """
    # Setup the test window
    testscr = curses.newwin(10, 10, 10, 10)
    
    # Draw a box around the test window and print a string
    
    testscr.addstr(1, 1, "Mobile Test Window! 1, 2, 3, A, B, C")
    testscr.box()
    testscr.refresh()
    """

    
    # Wait for a keypress
    stdscr.getkey()
    

def refresher(artscr, textscr, stdscr):
    artscr.refresh()
    textscr.refresh()
    stdscr.refresh()


def menu(textscr):
    ...

def ascii_scan():
    f = open("art.txt", "r")
    print(f.read())



# Setting up the wrapper so that we play nice with the terminal
def main():
   curses.wrapper(panes)
   ascii_scan() 

# Call the main function
main()
