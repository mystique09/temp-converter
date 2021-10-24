use std::io;

fn program_banner() -> () {
  println!("Temperature converter\nCelcius to Farenheit and Farenheit to Celcius\n")
}

fn main() {
  program_banner();

  loop {
    let mut temp_inp: String = String::new();

    println!("Enter a number to convert: ");

    io::stdin()
    .read_line(&mut temp_inp)
    .expect("ERROR: cannot read line");

    let temp_inp: f32 = match temp_inp.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid input, number expected!");
        continue
      },
    };
    
    let mut temp: String = String::new();
    
    println!("Enter the desired Temperature conversion, \n\n('cel' | 'celcius' | 'c') for Farenheit to Celcius and \n\n('faren' | 'farenheit' | 'f') for Celcius to Farenheit.\n");
    
    io::stdin()
    .read_line(&mut temp)
    .expect("Cannot read line!");
    
    let temp = temp.trim();
    
    match temp {
      "celcius" | "c" | "celc" => {
      println!("Result of Farenheit to Celcius conversion:\n{}", (temp_inp - 32.0) / 1.8);
        break;
      },
      "farenheit" | "faren" | "f" => {
      println!("Result of Farenheit to Celcius conversion:\n{}", (temp_inp * 1.8) + 32.0);
      break;
      },
      _ => {
        println!("Invalid keyword: {}", temp);
        continue;
      }
    }
  }
}