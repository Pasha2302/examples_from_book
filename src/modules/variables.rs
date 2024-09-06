

pub fn _var_example() {
    const MAX_POINTS: u32 = 100_000;
    let mut x: i32 = 1;
    let my_name: &str = "Pasha";
    println!("Значение мутабельной (изменяемой по значению) переменной x = {}", x);
    
    x = 2;
    println!("Новое Значение x = {}", x);
    println!("Не изменяемая (не мутабельная) переменная My Name = {}", my_name);
    println!("Значение константы MAX_POINTS (не может быть <mut>) = {}", MAX_POINTS);

    // Затенение:
    let y: u8 = 5;
    let y: u8 = y + 1;
    let y: u8 = y * 2;
    println!("Значение преременной < y > после затенения = {}", y);

    // Тип char равен четырем байтам:
    let symbol_c: char = 'c';
    let symbol_z: char = 'ƶ';
    let symbol_cat: char = '😻';
    println!(
        "Это тип данных символы: {}, {}, {} .",
        symbol_c, symbol_z, symbol_cat
    );

    // Составные типы:

    // - Кортеж:
    let tuple_type: (i32, f64, i32) = (500, 6.1, 1);
    // деструктурировать значения кор­тежа:
    let (tup_x, tup_y, tup_z) = tuple_type;
    println!(
        "\nДеструктурированные значения:
         (tup_y) = {} (tup_x) = {} (tup_z) = {}",
        tup_y, tup_x, tup_z
    );
    println!(
        "\nОбращаемся к элементу кортежа с помощью точечной анатации:
         < tuple_type.1 >: {}", tuple_type.1
    );

    // Масив
    // Массивы полезны, если вы хотите, чтобы данные размещались в стеке, а не в куче:
    let list_number: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    println!("\nТип масив < list_number > = {:?}", list_number);
    // println!(
    //     "Тип масив < list_number (вертикальный вывод в косоль) > = {:#?}",
    //     list_number
    // );

    let months = [
        "Январь", "Февраль", "Март", "Апрель",
        "Май", "Июнь", "Июль", "Август",
        "Сентябрь", "Октябрь", "Ноябрь", "Декабрь",
    ];

    println!("Масив с названиями месяцев < months > = {:#?} .", months);
    let list_a = ['a'; 5]; // создать масив из пяти символов 'a'.
    println!("Масив < list_a > {:?}", list_a);
    
    // Доступ к элементам масива:
    println!("\n < months[2] > = {}", months[2]);
    // Типы isize или usize по преимуществу используются в индексировании коллекции.
    let index: usize = 10;
    println!("\n < months[index] > = {}", months[index]);

}
