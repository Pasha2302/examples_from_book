// Cargo.toml
    // [dependencies]
    // rand = "0.3.14" (или другую версию)
// Установка >> cargo build
use rand::Rng;


pub fn _elseif_example() {
    // Выражения if
    let number: u32 = rand::thread_rng().gen_range(1..=100);

    if number < 17 {
        println!("\nУсловие ( if number < 5 ) было истинным. Namber = {}", number);
    } else {
        println!("\nУсловие ( if number < 5 ) было ложным. Namber = {}", number);
    };

    // if number {
    //     println!("ВОзможно number не приведется к типу < bool > автоматически ...");
    // };

    if number != 0 { println!("\nЧисло было отличным от нуля ..."); };

    println!("\n\nОбработка нескольких условий с помощью else if:");
    if number % 4 == 0 {
        println!("Число делится на 4");
    } else if number % 3 == 0 {
        println!("Число делится на 3");
    } else if number % 2 == 0 {
        println!("Число делится на 2");
    }else {
        println!("Число не делится на 4, 3 и 2");
    }

    println!("\n\nИспользование выражения if в инструкции let");
    let condition = true;
    // значения, которые потенциально будут результатами из каждого ветвления if, должны иметь одинаковый тип.
    let number_2 = if condition {5} else {6};
    println!("Значение < number_2 > вычеслено как: {}", number_2);

}
