use rand::Rng;
use std::cmp::Ordering;
use std::io;


pub fn _start_game() {
    println!("Угадай число!");
    println!("Пожалуйста введите свою догадку.");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Секретное число равно {}", secret_number);
    
    loop {
        let mut guess: String = String::new();
        println!("Пожалуйста, введите свою догадкую.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Не получилось прочитать строку.");

        // let guess: u32 = guess.trim().parse()
        //     .expect("Пожалуйста, наберите число!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Вы загадали: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое число!"),
            Ordering::Greater => println!("Слишком большое число"),
            Ordering::Equal => {
                println!("Вы выиграли!!!");
                break;
            }
        };
    }
}
