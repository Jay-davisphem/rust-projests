const TWELVE_DAYS: [(&str, &str); 12] = [
    ("first", "A partridge in a pear tree"),
    ("second", "Two turtle doves"),
    ("third", "Three French hens"),
    ("fourth", "Four calling birds"),
    ("fifth", "Five golden rings"),
    ("sixth", "Six geese a-laying"),
    ("seventh", "Seven swans a-swimming"),
    ("eighth", "Eight maids a-milking"),
    ("ninth", "Nine ladies dancing"),
    ("tenth", "Ten lords a-leaping"),
    ("eleventh", "Eleven pipers piping"),
    ("twelfth", "Twelve drummers drumming"),
];

fn main() {
    for i in 0..TWELVE_DAYS.len() {
        let (day, _) = TWELVE_DAYS[i];

        display_generic_lyrics(day);

        let mut j = 0;
        while j <= i {
            let stringj = TWELVE_DAYS[i - j].1;
            print!("{stringj}");
            if i - j == 0 {
                println!(".");
            } else {
                println!(",");
            }
            j += 1;
        }
        println!();
    }
}

fn display_generic_lyrics(day: &str) {
    println!("On the {day} day of Christmas,\nmy true love gave to me")
}
