// let _hours = 77;
// let _months = ["Sprout", "Harvest", "Solstice", "Wilts", "Wending", "Void", "Thaw"];
// let _years = 7;
// let _era = String::from("Era of the Uncrowned King");

pub struct Calendar {
    hours: i32,
    days: Days,
    months: Month,
    years: i32,
    era: String
}

enum Days {
    First, 
    Second, 
    Third, 
    Fourth,
    Fifth, 
    Sixth, 
    Seventh,
}

enum Months {
    Sprout, 
    Harvest, 
    Solstice, 
    Wilts,
    Wending, 
    Void,
    Thaw,
}