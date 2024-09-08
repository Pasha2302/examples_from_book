

fn _start_example_loop() {
    // Цикл loop:
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Результат возвращенный циклом loop = {}", result);

    // Цикл While:
    let mut number: i32 = 3;
    while number != 0 {
        println!("number = {}", number);
        number = number - 1;
    }
    println!("Приехали!");

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;
    
    println!("=================================================================");
    while index < a.len() {
        println!("Значение по индексу [{}] = {}", index, a[index]);
        index += 1;
    }
    println!("=================================================================");
    // Цикл For:
    for element in a.iter() {
        println!("For Element = {}", element);
    }

    // Обратный отсчет с использованием цикла for:
    for i in (0..10).rev() {
        println!(">> [{}] <<", i);
        if i == 0 {println!("Boooom!!!")}
    }

    // Прямой отсчет с использованием цикла for:
    for i in (0..10).into_iter() {
        println!(">> [{}] <<", i);
        if i == 0 {println!("Boooom!!!")}
    }

}
