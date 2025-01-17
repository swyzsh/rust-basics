// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
  // An `enum` variant may either be `unit-like`,
  PageLoad,
  PageUnload,
  // like tuple structs,
  KeyPress(char),
  Paste(String),
  // or c-like structures.
  Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent::PageUnload => println!("page unloaded"),
    // Destructure `c` from inside the `enum` variant.
    WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
    WebEvent::Paste(s) => println!("pasted \"{}\".", s),
    // Destructure `Click` into `x` and `y`.
    WebEvent::Click { x, y } => {
      println!("clicked at x={}, y={}.", x, y);
    }
  }
}

// If you use a type alias, you can refer to each enum variant via its alias.
// This might be useful if the enum's name is too long or too generic, and you want to rename it.
enum VeryVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Subtract,
}
// The most common place you'll see this is in `impl` blocks using the `Self` alias.
impl VeryVerboseEnumOfThingsToDoWithNumbers {
  fn run(&self, x: i32, y: i32) -> i32 {
    match self {
      Self::Add => x + y,
      Self::Subtract => x - y,
    }
  }
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted = WebEvent::Paste("my text".to_owned());
  let click = WebEvent::Click { x: 20, y: 80 };
  let load = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  // We can refer to each variant via its alias, not its long and inconvenient name.
  let _x = Operations::Add;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);
}
