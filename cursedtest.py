import curses

def main():


    """
    Using the wrapper so it doesn't fuck up our terminal
    """

    curses.wrapper(curses_main)

def curses_main(w):

    """
    Called curses_main to emphasize that it's the logical
    but not actual main function, which is curses.wrapper().

    Calls several other functions for demonstration.
    """
    
    w.addstr("------------\n")
    w.addstr(" __________ \n")
    w.addstr("/          \ \n")
    w.addstr("|  o    o  | \n")
    w.addstr("|     >    | \n")
    w.addstr("|    ___   | \n")
    w.addstr("|          | \n")
    w.addstr(" \ ______ /  \n")
    w.refresh()

    printing(w)

    moving_and_sleeping(w)

    coloring(w)

    w.addstr("\n press any key to exit...")
    w.refresh()
    w.getch()

def printing(w):

    """
    Some printing demos
    """

    w.addstr("This was printed using addstr()\n\n")
    w.refresh()

    w.addstr("The following letter was printed using addch:-")
    w.addch('a')
    w.refresh()

    w.addstr("\n\nThese numbers were printed using addstr:-\n{}\n{:.6f}\n".format(123, 456.789))
    w.refresh()

def moving_and_sleeping(w):

    """
    Demonstrates moving the cursor before printing
    """

    row = 5
    col = 0

    curses.curs_set(False)

    for c in range(65, 91):
        
        w.addstr(row, col, chr(c))
        w.refresh()
        row += 1
        col += 1
        curses.napms(100)

    curses.curs_set(True)

    w.addch('\n')

def coloring(w):
    
    """
    Demo of BG and FG colors.
    """

    if curses.has_colors():
        
        curses.init_pair(1, curses.COLOR_YELLOW, curses.COLOR_RED)
        curses.init_pair(2, curses.COLOR_GREEN, curses.COLOR_GREEN)
        curses.init_pair(3, curses.COLOR_MAGENTA, curses.COLOR_CYAN)

        w.addstr("Yellow on red\n\n", curses.color_pair(1))
        w.refresh()

    else:
        
        w.addstr("has_colors() = False\n");
        w.refresh()


main()


