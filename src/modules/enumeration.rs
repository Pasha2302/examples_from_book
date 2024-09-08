
// Определение перечисления:
#[derive(Debug)]
enum _IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


// Это перечисление имеет четыре варианта с разными типами:
//  Quit вообще не имеет связанных с ним данных.
//  Move включает в себя анонимную структуру.136
//  Write включает в себя одно значение String.
//  ChangeColor включает в себя три значения i32.
#[derive(Debug)]
enum _Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl _Message {
    fn _call(&self) {
        // Тело метода ...
        ()
    }
}


// Перечисление Coin, в котором вариант Quarter также имеет значение UsState
#[derive(Debug)]
enum _UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum _Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(_UsState),
}

// Выражение match как оператор управления потоком:
fn _value_in_cents(coin: &_Coin) -> u8 {
    match coin {
        _Coin::Penny => 1,
        _Coin::Nickel => 5,
        _Coin::Dime => 10,
        _Coin::Quarter(state) => {
            println!("Четвертак из штата {:?}!", state);
            25
        },

    }
}

fn _pluse_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn _start_example_enum() {
    let home: _IpAddr = _IpAddr::V4(127, 0, 0, 1);
    let loopback: _IpAddr = _IpAddr::V6(String::from("::1"));
    println!("IP Home: {:?}", home);
    println!("IP Loopback: {:?}", loopback);

    let message1: _Message = _Message::Write(String::from("Hello"));
    let message2: _Message = _Message::Quit;
    println!("\nПеречисление <message1> Типа <Message>: result: {:?}", message1);
    println!("\nПеречисление <message2> Типа <Message>: result: {:?}", message2);

    // Перечисление Option:
    // Варианты перечисления типа Option доступны глобально и их не нужно импортировать как и сам Option
    // Option::Some - используется для наглядности, так же можноиспользовать просто Some(T)
    let _some_number: Option<i32> = Option::Some(5);
    let _some_string: Option<&str> = Option::Some("Строковой литерал");
    let _adsent_number: Option<i32> = None;

    let coin = _Coin::Quarter(_UsState::Alabama);
    println!("\nЭкземпляр перечисления <coin>: {:?}", coin);
    println!("value_in_cents(coin): {}", _value_in_cents(&coin));

    

    let five: Option<i32> = Some(5);
    let six: Option<i32> = _pluse_one(five);
    let none: Option<i32> = _pluse_one(None);
    println!("\n six = {:?}, none = {:?}", six, none);

    // Заполнитель _:
    // Тем самым мы перечислили в паттерне все возможные варианты.
    let some_u8_value: u8 = 5;
    match some_u8_value {
        1 => println!("один"),
        3 => println!("три"),
        5 => println!("пять"),
        7 => println!("семь"),
        _ => (),
    }

    // Обрабатываем единственный вариант.
    let some_u8_value: Option<u8> = Some(3u8);
    match some_u8_value {
        Some(3) => println!("три"),
        _ => (),
    }

    // Сжатое управление потоком с помощью if let:
    // Синтаксис if let позволяет кратко совместить if и let для обработки значений,
    // которые совпадают с одним паттерном, игнорируя остальные.
    if let Some(3) = some_u8_value {
        println!("ТРИ!!!");
    }

    let mut _count: i32 = 0;
    match &coin {
        _Coin::Quarter(state) => println!("Четвертак из штата {:?}!", state),
        _ => _count += 1,
    }
    // Или мы могли бы использовать выражения if let и else:
    _count = 0;
    if let _Coin::Quarter(state) = coin {
        println!("Четвертак из штата {:?}!", state);
    } else {
        _count += 1;
    }

}
