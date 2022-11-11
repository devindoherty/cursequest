// let _hours = 77;
// let _months = ["Sprout", "Harvest", "Solstice", "Wilts", "Wending", "Void", "Thaw"];
// let _years = 7;
// let _era = String::from("Era of the Uncrowned King");

pub struct Calendar<'a> {
    hours: i32,
    days: [&'a str; 7],
    months: [&'a str; 7],
    years: i32,
    era: String
}

impl Calendar<'_> {

    fn new() -> Calendar<'static> {
        Calendar {
            hours: 1,
            days: ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh"],
            months: ["Sprout", "Harvest", "Solstice", "Wilts", "Wending", "Void", "Thaw"],
            years: 1,
            era: String::from("Era of the Uncrowned King"),
        }
    }
    
    fn set_date(){

    }
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