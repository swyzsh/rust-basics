struct Number {
  odd: bool,
  value: i32,
}

// instead of :

// fn print_number(n: Number) {
//   if n.odd == true {
//     println!("Odd number: {}", n.value);
//   } else {
//     println!("Even number: {}", n.value);
//   }
// }

fn print_number(n: Number) {
  if let Number { odd: true, value } = n {
    println!("Odd number: {}", value);
  } else if let Number { odd: false, value } = n {
    println!("Even number: {}", value)
  }
}

fn main() {
  let one = Number { odd: true, value: 1 };
  let two = Number { odd: false, value: 2 };
  print_number(one);
  print_number(two);
}
