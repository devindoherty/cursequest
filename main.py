import curses
import time
import atlas


MENU_CHOICE = {
        0: "travel",
        1: "explore",
        2: "interact",
        3: "character",
        4: "journal",
        5: "quit",
        }


ASCII_ENTER = 10


# Setting up two windows, one for ASCII art and maps and whatnot, the other for text output/input
def screens(stdscr):
    height, width = stdscr.getmaxyx()
    curses.start_color()

    # Finding starting y,x for art and text panes
    art_x = 0
    art_y = 0
    text_x = 0
    text_y = int((height // 2) + 1)

    # Setup the Art Pane with artscr
    artscr = curses.newwin(int(height // 2) - 1, width, art_y, art_x)
    curses.init_pair(2, curses.COLOR_RED, curses.COLOR_BLACK)
    artscr.addstr(1, 1, "This is the Art Window", curses.color_pair(2))
    artscr.box()
    artscr.refresh()

    # Setup the Text Pane with textscr
    textscr = curses.newwin(int((height // 2) - 1) , width, text_y, text_x)
    textscr.addstr(1, 1, "This is the Text Window", curses.color_pair(2))
    textscr.keypad(True)
    textscr.box()
    textscr.refresh()

#    # Setup the test window
#    testscr = curses.newwin(10, 10, 10, 10)
#    testscr.addstr(1, 1, "Mobile Test Window! 1, 2, 3, A, B, C")
#    testscr.box()
#    testscr.refresh()

    return textscr, artscr #testscr

# Determining menu placement, formatting menu
def menu(textscr, current_row, menu_options):
    curses.init_pair(1, curses.COLOR_BLACK, curses.COLOR_WHITE)

    text_height, text_width = textscr.getmaxyx()

    for idx, element in enumerate(menu_options):
        menu_y = (text_height // 2) + idx  
        menu_x = (text_width // 2) - (len(element) // 2) 

        if idx == current_row:
            textscr.attron(curses.color_pair(1))
            textscr.addstr(menu_y, menu_x, element)
            textscr.attroff(curses.color_pair(1))

        else:
            textscr.addstr(menu_y, menu_x, element)

    textscr.refresh()


def draw_art(artscr):
    f = open("art.txt", "r")
    print(f.read())

    # Reading external files STILL BROKEN ARGHHHH

    with open("art.txt", "r", encoding ="utf8") as f:
        lines = f.readlines()

    for a in lines:
        artscr.addstr(0, 0, a.rstrip())
        artscr.refresh()


# Color Testing
def color_test(textscr):
    curses.start_color()
    curses.use_default_colors()
    for i in range(0, curses.COLORS):
        curses.init_pair(i + 1, i, -1)
    try:
        for i in range(0, 255):
           textscr.addstr(str(i), curses.color_pair(i))
    except curses.ERR:
        pass

   
def curses_main(stdscr):
    # Blank the canvas
    stdscr = curses.initscr()
    stdscr.clear()
    stdscr.refresh()

    # Border for entire terminal
    #stdscr.box()


    # Finding Terminal size. Height = y, width = x
    height, width = stdscr.getmaxyx()

    textscr, artscr = screens(stdscr)

    color_test(textscr)

    # Setup the menu and call menu function

    current_row = 0   
    menu_options = ["T - Travel",
                    "E - Explore",
                    "I - Interact",
                    "C - Character",
                    "J - Journal",
                    "Q - Quit"]

    menu(textscr, current_row, menu_options)

    # Menu Loop 
    while True:
        key = textscr.getch()
        menu_selection = MENU_CHOICE[current_row]

        if key == curses.KEY_UP and current_row > 0:
            current_row -= 1
            artscr.addstr(19, 5, "UP KEY PRESSED")

        elif key == curses.KEY_DOWN and current_row < 5:
            current_row += 1
            artscr.addstr(18, 5, "DOWN KEY PRESSED")

        elif key == ord("t") or key == ASCII_ENTER and menu_selection == "travel":
            travel(artscr, textscr, stdscr)

        elif key == ord("e") or key == ASCII_ENTER and menu_selection == "explore":
            explore(artscr)

        elif key == ord("i") or key == ASCII_ENTER and menu_selection == "interact":
            interact(textscr)

        elif key == ord("c") or key == ASCII_ENTER and menu_selection == "character":
            character(artscr, textscr)

        elif key == ord("j") or key == ASCII_ENTER and menu_selection == "journal":
            journal(artscr)

        elif key == ord("q") or key == ASCII_ENTER and menu_selection == "quit":
            break

        menu(textscr, current_row, menu_options)

        textscr.refresh()
        artscr.refresh()



def travel(textscr, artscr, stdscr):
    textscr, artscr = screens(stdscr)
    art_height, art_width = artscr.getmaxyx()
    text_height, text_width = textscr.getmaxyx()
    world_map = atlas.world_map

    player_y = 11
    player_x = 11
    playeryx = player_y, player_x

    artscr.addstr(art_height // 2, art_width // 2, "YE OLDE MAP OF KLATHIA")
    textscr.addstr(text_height // 2, text_width // 2, "Arrow or WASD Keys to Move")
    textscr.addstr(text_height // 2 + 1, text_width //2, "Q to return to Main Menu")
    atlas.draw_map(artscr, world_map)

    while True:
        key = textscr.getch()

        if key == curses.KEY_UP:
            player_y -= 1
            artscr.addstr(20, 6, "UP KEY PRESSED")

        elif key == curses.KEY_DOWN:
            player_y += 1

        elif key == curses.KEY_LEFT:
            player_x -= 1

        elif key == curses.KEY_RIGHT:
            player_x += 1

        elif key == ord("q"):
            textscr.clear()
            break


        atlas.draw_map(artscr, world_map)
        atlas.draw_player_position(artscr, player_y, player_x)
        artscr.refresh()

def explore(artscr):
    artscr.addstr(4, 3, "EXPLORATION ART")

def interact(textscr):
    textscr.erase()
    textscr.refresh()

    current_row = 0
    menu_selection = MENU_CHOICE[current_row]
    menu_options = ["Talk",
                    "Fight",
                    "Explore"]

    menu(textscr, current_row, menu_options)


    while True:
        key = textscr.getch()

        if key == curses.KEY_UP and current_row > 0:
            current_row -= 1

        elif key == curses.KEY_DOWN and current_row < 5:
            current_row += 1


        menu(textscr, current_row, menu_options)

    textscr.addstr("INTERACTION OPTIONS MENU")

def character(textscr, artscr):
    art_height, art_width = artscr.getmaxyx()
    text_height, text_width = textscr.getmaxyx()
    current_row = 0

    menu_selection = MENU_CHOICE[current_row]
    menu_options = ["Inventory",
                    "Stats",
                    "Skills",
                    "Back"]

    menu(textscr, current_row, menu_options)

    def character_status():
        ability_scores = base, modifiers, bonus
        base = 0
        modifers = 0
        bonus = 0

        Body = statistic #Covers strength and speed
        Mind = statistic #Knowledge and wisdom
        Soul = statistic #Personality and charisma

    while True:
        key = textscr.getch()

        if key == ord("s"):
            artscr.addstr((art_height // 2) + 1, (art_width // 2), "-------------")
            artscr.addstr((art_height // 2) + 2, (art_width // 2), " __________ \n")
            artscr.addstr((art_height // 2) + 3, (art_width // 2), "/          \ \n")
            artscr.addstr((art_height // 2) + 4, (art_width // 2), "|  o    o  | \n")
            artscr.addstr((art_height // 2) + 5, (art_width // 2), "|     >    | \n")
            artscr.addstr((art_height // 2) + 6, (art_width // 2), "|    ___   | \n")
            artscr.addstr((art_height // 2) + 7, (art_width // 2), "|          | \n")
            artscr.addstr((art_height // 2) + 8, (art_width // 2), " \ ______ /  \n")
            artscr.addstr((art_height // 2) + 9, (art_width // 2), "-------------\n")
            artscr.addstr((art_height // 2) + 10, (art_width // 2),"KRYLL the KLATHIAN\n")

        if key == ord("c"):
            character_status()






def journal(artscr):
    draw_art(artscr)





class MapScreen:
    pass

class ExploreScreen:
    pass

class CharacterScreen:
    pass




# Setting up the wrapper so that we play nice with the terminal
def main():
   curses.wrapper(curses_main)

# Call the main function
if __name__ == "__main__":
    main()

