import curses
import time
import os
import sys


# Setting up two windows, one for ASCII art and maps and whatnot, the other for text output/input
def panes(stdscr):
    
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
    artscr.refresh()

    # Setup the Text Pane with textscr
    textscr = curses.newwin(int((height // 2) - 1) , width, text_y, text_x)
    textscr.addstr(1, 1, "This is the Text Window")  
    textscr.keypad(True)
    textscr.box()
    textscr.refresh()

    return textscr, artscr
    
def travel(artscr):
    artscr.addstr(3, 3, "KLATHIA")


def explore(artscr):
    pass


def interact(artscr):
    pass

def character(artscr):
    pass

def journal(artscr):
    pass


# Determining menu placement, formatting menu
# menu_y, menu_x placement
# enumeration for format

def menu(textscr, current_row):
    
    curses.init_pair(1, curses.COLOR_BLACK, curses.COLOR_WHITE)
    
    text_height, text_width = textscr.getmaxyx()

    menu = ["T - Travel", "E - Explore", "I - Interact", "C - Character", "J - Journal", "Q - Quit"]

    for idx, element in enumerate(menu):
        menu_y = (text_height // 2) + idx  
        menu_x = (text_width // 2) - (len(element) // 2) 

        if idx == current_row:
            textscr.attron(curses.color_pair(1))
            textscr.addstr(menu_y, menu_x, element)
            textscr.attroff(curses.color_pair(1))

        else:
            textscr.addstr(menu_y, menu_x, element)

    textscr.refresh()


#def ascii_scan():
#    f = open("art.txt", "r")
#    print(f.read())
#
#    # Reading external files STILL BROKEN ARGHHHH
#
#    with open("art.txt", "r", encoding ="utf8") as f:
#        lines = f.readlines()
#
#    for a in lines:
#        artscr.addstr(20, 5, a.rstrip())
#        artscr.refresh()
#
#
#    # Color Testing
#    curses.start_color()
#    curses.use_default_colors()
#    for i in range(0, curses.COLORS):
#        curses.init_pair(i + 1, i, -1)
#    try:
#        for i in range(0, 255):
#           textscr.addstr(str(i), curses.color_pair(i))
#    except curses.ERR:
#        pass
#
#
#    # Print our different windows
#
#
#   
#    # Setup the test window
#    testscr = curses.newwin(10, 10, 10, 10)
#    
#    # Draw a box around the test window and print a string
#    
#    testscr.addstr(1, 1, "Mobile Test Window! 1, 2, 3, A, B, C")
#    testscr.box()
#    testscr.refresh()
   

def curses_main(stdscr):
     # Blank the canvas
    stdscr = curses.initscr()
    stdscr.clear()
    stdscr.refresh() 
    
    # Border for entire terminal
    #stdscr.box()


    # Finding Terminal size. Height = y, width = x
    height, width = stdscr.getmaxyx()

    panes(stdscr)
    textscr = panes(stdscr)
    artscr = panes(stdscr)
    
    # Setup the menu and call menu function

    current_row = 0   

    menu(textscr, current_row)

    # Menu Loop 
    while True:
        key = textscr.getch()

        if key == curses.KEY_UP and current_row > 0:
           current_row -= 1
           artscr.addstr(19, 5, "UP KEY PRESSED")

        elif key == curses.KEY_DOWN and current_row < 5:
            current_row += 1
            artscr.addstr(18, 5, "DOWN KEY PRESSES")
               
        elif key == curses.KEY_ENTER:
           artscr.addstr(17, 5, "YOU HAVE BEGUN YOUR JOURNEY")
   
        elif key == ord("t"):
            artscr.addstr(20, 5, "YE OLD MAP")
            travel(artscr)

        elif key == ord("e"):
            explore(artscr)

        elif key == ord("i"):
            interact(artscr)

        elif key == ord("c"):
            character(artscr)

        elif key == ord("j"):
            journal(artscr)

        elif key == ord("q"):
            break
        
        menu(textscr, current_row)
   
        textscr.refresh()
        artscr.refresh()



# Setting up the wrapper so that we play nice with the terminal
def main():
   curses.wrapper(curses_main)

# Call the 'main' function
main()
