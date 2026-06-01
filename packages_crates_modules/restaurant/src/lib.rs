// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

mod front_of_house {
pub mod hosting { // we add pub here to make hosting public, otherwise we won't be able to access it from eat_at_restaurant
pub fn add_to_waitlist() {} // we add pub here to make add_to_waitlist public, otherwise we won't be able to access it from eat_at_restaurant
fn seat_at_table() {}
}
mod serving {
fn take_order() {}
fn serve_order() {}
fn take_payment() {}
}
}

// The `use` keyword creates a shortcut, but ONLY for the specific scope it is placed in.
// If the code that uses this shortcut is moved into a child module, this line 
// will trigger an "unused import" warning because the child module has its own scope.
use crate::front_of_house::hosting; // we add this use statement to bring hosting into scope, so we can use hosting::add_to_waitlist instead of front_of_house::hosting::add_to_waitlist in eat_at_restaurant

// ==========================================
// RUST `use` PATH CONVENTIONS (Cheat Sheet)
// ==========================================

// RULE 1: For functions, bring the PARENT module into scope.
// This makes it clear the function isn't local, while keeping the call short.
// Example:
// use crate::front_of_house::hosting;
// hosting::add_to_waitlist(); 

// RULE 2: For structs, enums, and other items, bring the FULL PATH into scope.
// This is simply the standard Rust community convention.
// Example:
// use std::collections::HashMap;
// let mut map = HashMap::new();

// RULE 3 (EXCEPTION): Name Collisions. 
// If two items have the same name, bring their parent modules into scope to tell them apart.
// Example:
// use std::fmt;
// use std::io;
// fn func1() -> fmt::Result { ... }
// fn func2() -> io::Result<()> { ... }

// ==========================================

// ==========================================
// RUST ALIASING CONVENTION: The `as` Keyword
// ==========================================

// If you have a name collision, you don't *have* to use the parent modules. 
// Instead, you can import the exact items and rename them locally using `as`.
// This creates an alias that only exists in this specific scope.

// Example:
// use std::fmt::Result; 
// use std::io::Result as IoResult; // Renames this Result to 'IoResult' locally

// fn function1() -> Result { ... }     // Uses the std::fmt version
// fn function2() -> IoResult<()> { ... } // Uses the std::io version


// ==========================================
// RE-EXPORTING: The `pub use` Keyword
// ==========================================

// Standard `use` brings an item into scope PRIVATELY. 
// `pub use` brings an item into scope AND makes it PUBLIC for outside code to use.

// WHY DO THIS? (Internal Structure vs. Public API)
// It allows you to organize your internal code however makes sense to you 
// (like deeply nested `front_of_house` and `back_of_house` modules), 
// but expose a clean, simple, flat structure to the people actually using your code.

// Example Result:
// Instead of forcing external users to type: 
// restaurant::front_of_house::hosting::add_to_waitlist();
// (which would also require making the `front_of_house` module public)
//
// `pub use` lets them just type:
// restaurant::hosting::add_to_waitlist();

// ==========================================
// IMPORTING EXTERNAL PACKAGES & STD LIBRARY
// ==========================================

// EXTERNAL CRATES (from crates.io)
// Step 1: Must be added to Cargo.toml first (e.g., rand = "0.8.5")
// Step 2: Bring into scope using the crate's name.
// use rand::Rng;

// THE STANDARD LIBRARY (std)
// Step 1: Built into Rust, so it is NEVER added to Cargo.toml.
// Step 2: Bring into scope using the `std::` prefix.
// use std::collections::HashMap;

// ==========================================
// CLEANING UP IMPORTS: Nested Paths
// ==========================================

// RULE 1: Group Shared Prefixes
// If multiple imports start with the same path, combine them using `{}` to save space.
// 
// Instead of this:
// use std::cmp::Ordering;
// use std::io;
// 
// Do this:
// use std::{cmp::Ordering, io};

// RULE 2: The `self` Keyword
// If you want to import a module AND an item inside that exact module, 
// use `self` to represent the parent module itself.
//
// Instead of this:
// use std::io;
// use std::io::Write;
//
// Do this:
// use std::io::{self, Write};

// ==========================================

// ==========================================
// RUST MULTI-FILE MODULE SYSTEM 
// ==========================================

// RULE 1: `mod` is a signpost, not an include.
// Writing `mod my_module;` tells the compiler to look for a file 
// named `my_module.rs` in the same directory.

// RULE 2: The filesystem must match the module tree.
// - Level 1 (Child of lib.rs/main.rs): Code goes in `src/my_module.rs`
// - Level 2 (Nested child): Code goes in a folder named after the parent!
//   Example: A `hosting` module inside `front_of_house` must be placed at:
//   `src/front_of_house/hosting.rs`

// SUMMARY OF STEPS TO MOVE A MODULE OUT OF THIS FILE:
// 1. Replace the `mod name { ... }` block with `mod name;`
// 2. Create `name.rs` and paste the code there.
// 3. If moving a nested module, create a folder named after the parent file, 
//    and put the new child `.rs` file inside that folder.
// ==========================================

// ==========================================
// ALTERNATE FILEPATHS: The `mod.rs` Style
// ==========================================

// Rust supports two ways to map modules to files. Both work perfectly!

// 1. THE NEW STYLE (Recommended)
// Avoids having 50 files all named "mod.rs" open in your editor.
// - Root module:   `src/my_module.rs`
// - Child module:  `src/my_module/child.rs`

// 2. THE OLD STYLE (`mod.rs`)
// You will see this constantly in older crates and GitHub repos.
// - Root module:   `src/my_module/mod.rs`
// - Child module:  `src/my_module/child/mod.rs`

// RULES TO REMEMBER:

// - Do NOT mix both styles for the exact SAME module (it won't compile).
// - `mod` maps to files and builds the tree. 
// - `use` just creates shortcuts. Changing file names/locations 
//   doesn't change your `use` statements as long as the logical tree is the same!
// ==========================================

pub fn eat_at_restaurant() {
// absolute path
// we can use the crate keyword to start an absolute path because add_to_waitlist is defined in the same crate as eat_at_restaurant
//Preferred approach: Use this if the caller (eat_at_restaurant) and
// the definition (add_to_waitlist) are likely to be moved independently of each other.
// Starts from the crate root.
crate::front_of_house::hosting::add_to_waitlist();
// relative path
// we start at front_of_house because it's a sibling of eat_at_restaurant (defined at the same level of the module tree as eat_at_restaurant)
// Alternative approach: Use this if the caller and the definition 
// are tightly coupled and likely to be moved together into a new module.
front_of_house::hosting::add_to_waitlist();

// If this was moved to a different module without bringing the `use` statement into that child 
// module, it won't compile ("undeclared module"). 
// FIX: Either move the `use` statement inside the new module, 
// or change this path to use `super::hosting::add_to_waitlist()`.
hosting::add_to_waitlist();// we can use hosting::add_to_waitlist instead of front_of_house::hosting::add_to_waitlist because we brought hosting into scope with the use statement at the top of this file

// order a breakfast in the summer with Rye toast
let mut meal = back_of_house::Breakfast::summer("Rye");
// change our mind about what bread we'd like
meal.toast = String::from("Wheat");
println!("I'd like {} toast please", meal.toast);
// the next line won't compile if we uncomment it; we're not
// allowed to see or modify the seasonal fruit that comes
// with the meal
// meal.seasonal_fruit = String::from("blueberries");

let order1 = back_of_house::Appetizer::Soup;
let order2 = back_of_house::Appetizer::Salad;

}

fn deliver_order() {}
mod back_of_house {
fn fix_incorrect_order() {
cook_order();

// Relative path using `super` (acts like `../` in a filesystem).
// We use it here so that if `back_of_house` and `deliver_order` 
// are ever moved together to a new module, their parent-child 
// relationship stays the same and this call won't break.
super::deliver_order();

}
fn cook_order() {}

pub struct Breakfast { // we add pub here to make Breakfast public, otherwise we won't be able to access it from eat_at_restaurant
pub toast: String, // we add pub here to make toast public, otherwise we won't be able to access it from eat_at_restaurant
seasonal_fruit: String, // we don't add pub here because we don't want to make seasonal_fruit public, because available fruit changes quickly,
                        //so customers can't choose the fruit or even see what fruit they'll get until they get their breakfast
}
impl Breakfast {
pub fn summer(toast: &str) -> Breakfast { // we add pub here to make summer public, otherwise we won't be able to access it from eat_at_restaurant
Breakfast {
toast: String::from(toast),
seasonal_fruit: String::from("peaches"),
}
}
}

pub enum Appetizer { // we add pub here to make Appetizer public, otherwise we won't be able to access it from eat_at_restaurant
Soup, // we don't need to add pub here because the variants of an enum are public if the enum is public, so we can access Soup and Salad from eat_at_restaurant
Salad,
}

}
