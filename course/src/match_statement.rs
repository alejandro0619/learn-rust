// Rust has an extremely powerful control flow operator called match 
// that allows you to compare a value against a series of pattern matches. 
// Patterns can be made up of literal values, variable names, wildcards and many other things. 

// Think of a match expression as being like a coin-sorting machine: 
// coins slide down a track with variously sized hols along it,
// and each coin falls through the first hole it encounters that fits into. In the same way, 
// values go through each pattern in a match, and at the first pattern the value fits, 
// the value falls into the associated code block to be used during execution.

// Let's do some code:

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> i32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

// Match with Option<T>
// Let's say we wanted to get the inner T value out of the Some case when using Option<T>
// We can also handle Option<T> using match as we did with the Coin enum
// Instead of comparing coins we'll compare Option<T> variants.

// Let's create a function that gets a Option<i32> param
// If Some(number) let's add one to it. If None, return None.

pub fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(number) => Some(number + 1),
  }
}


// ~ Catch all pattern and the _ placeholder.

// Let's look an example where we wan to take special actions for a few particular values, but for all other values take one default action. 
//Imagine we are implementing a game where if you get a value of 3 on a dic roll, your player doesn't move, but instead gets a new fancy hat. 
// If you roll a 7, your player loses a fancy hat. For all other values, your player moves that number of spaces on the game board. Here's a match that implements that logic, with the result of the dice roll hardcoded rather than a random ovalue, and all other logic represented by functions without bodies because acrually implementing them is out of scope for this example.

fn game_test () {
  let dice_roll = 9;
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => move_player()
  }

  fn add_fancy_hat() {}
  fn remove_fancy_hat() {}
  fn move_player() {}
}

// _ is a special catch-all pattern that matches any value and does not bind to that value