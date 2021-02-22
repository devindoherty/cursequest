import curses

# Serves as the master window

screen = curses.initscr()


# Defining a new window

my_window = curses.newwin(15, 20, 0, 0)
stat_window = curses.newwin(15, 25, 30, 0)

# Printing strings within that new window
my_window.addstr("Hellow from default")
my_window.addstr(4, 4, "Hello from 4, 4")
my_window.addstr(5, 15, "Hello from 5, 15 with a loonnnnnng string")
my_window.box()



height, width = screen.getmaxyx()

stat_window.addstr("Width in columns:")
stat_window.addstr("Height in rows:" )






# Printing the window to the screen

my_window.refresh()
stat_window.refresh()
curses.napms(2000)


# Clearing the entire screen

screen.clear()
screen.refresh()

# Moving the window and putting it back on the screen

my_window.mvwin(10, 10)
stat_window.mvwin(30, 15)
my_window.refresh()
curses.napms(1000)

# Clear the window and redraw over the current window space

my_window.clear()
stat_window.clear()
my_window.refresh()
stat_window.refresh()
curses.napms(1000)

curses.endwin()
