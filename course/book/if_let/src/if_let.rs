// if let syntax lets you combine if and let into a less verbose
// way to handle values that match one pattern while ignoring the rest.
// Consider this following program that matches on a Option<u8>
// value in config_max variable.
// But we only want to execute code when a Some variant is reached.

fn test_verbose() {
  let config_max  = Some(3u8);
  match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
}

// Instead of writing all of the above code we can use the if let statement.

pub fn test_cool()-> Option<u8>{

  let config_max = Some(5u8);

  if let Some(max) = config_max {
    println!("The maximum is configured to be {:?}", max);
    return Some(max);
  } else {
    None
  }
}

// If let syntax means less typings, less boilerplate code. However we lose the exhaustive checking that match provides.
// if let can be thought of as a syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.