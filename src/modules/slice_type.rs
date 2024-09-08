use std::ptr;


fn first_word(s: &String) -> usize {
    // Функция first_word возвращает индекс первого слова в строке (оретируясь на пробелы между словами).
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return i };
    }
    s.len()
}

fn first_word2(s: &str) -> &str {
    // Функция first_word2 возвращает первое слово в строке (оретируясь на пробелы между словами).
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn test1 () {
    let x = 10;
    let y = &x; // y - это ссылка на x

    println!("x: {}, y: {}\n", x, y);
    // Пример изменяемой ссылки
    let mut x = 10;
    {
        let y: &mut i32 = &mut x; // изменяемая ссылка на x
        *y += 5;
    } // y больше не используется

    println!("x: {}", x);
}

fn test2 () {
    let mut x: i32 = 42;
    let x_ptr: *mut i32 = &mut x;

    unsafe {
        ptr::write(x_ptr, 100); // запись значения 100 по указателю x_ptr
        println!("Измененное значение по указателю: {}", x);
    }
}

fn test3() {
    let x: i32 = 10;
    let y: &i32 = &x; // y - ссылка на x
    let z: i32 = *y; // разыменование y для получения значения x
    println!("z: {}", z); // Output: 10
}


fn _start_exampe_slice() {
    // Срезовый тип:
    // Еще один тип данных, в котором не предусмотрено владение, — это срез (или от­резок).
    // Срезы позволяют ссылаться не на всю коллекцию, а на сплошную после­довательность элементов в коллекции.

    let my_name = String::from("Pasha Nebrat");
    // let mut my_name = String::from("Pasha Nebrat");

    let index_word: usize = first_word(&my_name);
    // my_name.clear(); Ошибки не будет и index_word будет не связан с данными, так как их уже не существует (это не надежно.)
    println!("Конец Индекса первого слова в строке <my_name>: {} ", index_word);

    // Строковые срезы
    // Строковый срез — это ссылка на часть значения типа String. Он выглядит следу­ющим образом:
    let slice_my_name: &str = first_word2(&my_name[..]); // передаем строковой срех, чтобы функция была более универсальной.
    // my_name.clear(); Вызовет ошибку компиляции, получение среза надежнее, так как связан с исходными данными.
    println!("Найденное первое слово в строке <my_name> типа <String>: {}", slice_my_name);

    let new_slice_str: &str = "Первое_слово остальные слова тут.";
    // Так как строковые литералы уже *являются* строковыми срезами,
    // это также работает без срезового синтаксиса!
    println!("\nНовый строковой срез: {}", first_word2(new_slice_str));

    let s: String = String::from("Hello World");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    let s2: &[u8; 11] = b"Hello World";
    let hello2: &[u8] = &s2[..5]; // аналогично [0..5]

    let s3: &str = "Hello World";
    let hello3: &str = &s3[0..5];

    println!("\n\nСсылка на срез байт строки b'Hello World'[0..5] = {:?}", hello2);
    println!("Ссылка на срез <String> Hello World[0..5] = {}", hello);
    println!("Ссылка на срез <String> Hello World[6..11] = {}", world);
    println!("Ссылка на срез <str> Hello World[0..5] = {}\n\n", hello3);

    test1();
    test2();
    test3();

    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[1..4]; // срез: [20, 30, 40]

    println!("\n\nSlice: {:?}", slice); // вывод: [20, 30, 40]
    println!("Pointer: {:p}", slice.as_ptr()); // вывод указателя
    println!("Length: {}\n\n", slice.len()); // вывод длины

}
