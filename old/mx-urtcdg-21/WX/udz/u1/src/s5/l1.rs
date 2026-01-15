/*
l1.ra
- This rust file is for writing learning patterns
*/

// -- Directives ---

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use yansi::Paint;

// --- Main Function ---

pub fn s5_l1_main() {
    greet();
}

// --- Sub Functions---

fn greet() {
    let str = "Secton 5 - Learning l1.rs";
    pswg(str.to_string());

    // Sub functions here
    pr_ab2();
}

/*
Learning Enums
1. Fist we will visualize what the enum should look like
2. This is like defining 3 different structs
3. In a case where you need similar structs use enums
*/

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String, year: i32 },
    Podcast(u32), // u32 is the episdoe number
    Placeholder,
}

// Implementation of every variants - On Implementation Block
impl Media {
    fn description1(&self) -> String {
        // Simple string print note the function is returning a scriing
        String::from("Media Description")
    }

    // Test Function for printiing stuff
    fn smell_panty(&self) -> String {
        String::from("Pussy Licker")
    }

    // Printing - execution based on teh varian
    fn description2(&self) -> String {
        // Is self a Book ?
        // Is self a Movie ?
        // Is self a AudioBook ?

        // Usign Match Statements - pttern Match statemnt
        match self {
            Media::Book { title, author } => {
                format!("Book: {} by {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} by {}", title, director)
            }
            Media::Audiobook { title, year } => {
                format!("Audiobook: {} published in {}", title, year)
            }
            Media::Podcast(id) => {
                format!("Podcast Episode: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

// Struct for catalog function

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    // Mimicking the Some option with a manual function
    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // good we have something to return
            Some(&self.items[index])
        } else {
            // Bad ! - Nothing to returb
            None
        }
    }
}

// enum MightHaveAValue<'a> {
//     ThereIsAValue(&'a Media),
//     NoValueAvailable,
// }

// Function to print out the enum
fn print_media(media: Media) {
    header("Printing Enums");
    println!("{:#?}", media.magenta())
}

// printing the audio book function
fn pr_ab() {
    header("Printing the Enum - Variants with match statemetns");

    // Audio Book
    let ab1 = Media::Audiobook {
        title: String::from("Booty Smelling"),
        year: 2099,
    };

    // Movie
    let mov1 = Media::Movie {
        title: String::from("Scat Mistress"),
        director: String::from("AssAddict"),
    };

    // Book
    let bok1 = Media::Book {
        title: String::from("BootyLicking"),
        author: String::from("Booty Addicts"),
    };

    // Podcast
    let pod1 = Media::Podcast(69);

    // Placeholder
    let ph1 = Media::Placeholder;

    // Printing the enums
    // print_media(ab1);
    // print_media(mov1);
    // print_media(bok1);

    // Printing the methods
    // println!("{}", ab1.description1().yellow());
    // println!("{}", mov1.description1().green());
    // println!("{}", bok1.description1().blue());

    // println!("{}", ab1.smell_panty().red())

    // using the match statement- Printing it out
    // println!("{}", ab1.description2().yellow());
    // println!("{}", mov1.description2().green());
    // println!("{}", bok1.description2().blue());

    // Make catalog - And Push items
    let mut catalog = Catalog::new();
    catalog.add(ab1);
    catalog.add(mov1);
    catalog.add(bok1);
    catalog.add(pod1);
    catalog.add(ph1);

    // println!("{:#?}", catalog);

    //-- Printing individual variants from the enum ---
    // println!("{:#?}", catalog.items.get(10));

    // Option is a built in use case
    //

    // match catalog.get_by_index(1) {
    //     Some(value) => {
    //         println!("Item : {:#?}", value);
    //     }
    //     None => {
    //         println!("No Value Available");
    //     }
    // }

    // This is for demonstartion the Enum Some and None
    // if let Some(value) = catalog.get_by_index(100) {
    //     println!("Item Pattern Match: {:#?}", value);
    // } else {
    //     println!("No Value Available");
    // }
}

fn pr_ab2() {
    header("Printing the Enum - Variants with match statemetns");

    // Audio Book
    let ab1 = Media::Audiobook {
        title: String::from("Booty Smelling"),
        year: 2099,
    };

    // Movie
    let mov1 = Media::Movie {
        title: String::from("Scat Mistress"),
        director: String::from("AssAddict"),
    };

    // Book
    let bok1 = Media::Book {
        title: String::from("BootyLicking"),
        author: String::from("Booty Addicts"),
    };

    // Podcast
    let pod1 = Media::Podcast(69);

    // Placeholder
    let ph1 = Media::Placeholder;

    // Make catalog - And Push items
    let mut catalog = Catalog::new();
    catalog.add(ab1);
    catalog.add(mov1);
    catalog.add(bok1);
    catalog.add(pod1);
    // catalog.add(ph1);

    let item = catalog.get_by_index(100);

    // println!("{}", "---Just printing the item---".yellow());
    // println!("{:#?}", item);

    // println!("{}", "--- Print item with unwarp ---".yellow());
    // println!("{:#?}", item.unwrap());

    // println!("{}", "--- Using Expect ---".yellow());
    // println!("{:#?}", item.expect("FUCK OFF - No Value"));

    println!("{}", "--- Using unwrap_or ---".yellow());
    println!("{:#?}", item.unwrap_or(&ph1));
}
