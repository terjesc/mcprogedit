#[derive(Clone, Debug, PartialEq)]
pub enum Colour {
    White = 0,
    Orange = 1,
    Magenta = 2,
    LightBlue = 3,
    Yellow = 4,
    Lime = 5,
    Pink = 6,
    Gray = 7,
    LightGray = 8,
    Cyan = 9,
    Purple = 10,
    Blue = 11,
    Brown = 12,
    Green = 13,
    Red = 14,
    Black = 15,
}

impl From<i32> for Colour {
    fn from(colour_number: i32) -> Self {
        match colour_number {
            0 => Colour::White,
            1 => Colour::Orange,
            2 => Colour::Magenta,
            3 => Colour::LightBlue,
            4 => Colour::Yellow,
            5 => Colour::Lime,
            6 => Colour::Pink,
            7 => Colour::Gray,
            8 => Colour::LightGray,
            9 => Colour::Cyan,
            10 => Colour::Purple,
            11 => Colour::Blue,
            12 => Colour::Brown,
            13 => Colour::Green,
            14 => Colour::Red,
            15 => Colour::Black,
            _ => panic!("Invalid colour number: {}", colour_number),
        }
    }
}

impl From<&str> for Colour {
    fn from(colour_string: &str) -> Self {
        match colour_string {
            "white" => Colour::White,
            "orange" => Colour::Orange,
            "magenta" => Colour::Magenta,
            "light_blue" => Colour::LightBlue,
            "yellow" => Colour::Yellow,
            "lime" => Colour::Lime,
            "pink" => Colour::Pink,
            "gray" => Colour::Gray,
            "light_gray" => Colour::LightGray,
            "cyan" => Colour::Cyan,
            "purple" => Colour::Purple,
            "blue" => Colour::Blue,
            "brown" => Colour::Brown,
            "green" => Colour::Green,
            "red" => Colour::Red,
            "black" => Colour::Black,
            _ => Colour::Black,
        }
    }

}
pub type Color = Colour;
