
// Добавление атрибута #[derive(Debug)]:
// Этот атрибут указывает компилятору автоматически генерировать реализацию трейта Debug для вашей структуры User.
// Реализация трейта Debug позволяет использовать спецификатор {:?} в макросе println! для отладочного вывода структуры.

// После добавления этого атрибута компилятор автоматически сгенерирует реализацию Debug для структуры User,
// и вы сможете выводить экземпляры этой структуры с помощью println! и спецификатора {:?}.

use std::fmt;

// Определение структуры User:
// #[derive(Debug)]
struct _User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Способ 2: Реализация трейта Display:
impl fmt::Display for _User {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User {{ user_name: {}, email: {}, sign_in_count: {}, active: {} }}",
        self.user_name, self.email, self.sign_in_count, self.active)
    }
}


fn _build_user(user_name: String, email: String) -> _User {
    // Функция build_user, которая принимает электронную почту и имя
    // пользователя и возвращает экземпляр структуры User

    // User {
    //     email: email,
    //     user_name: user_name,
    //     active: true,
    //     sign_in_count: 1,
    // }

    // Использование краткой инициализации полей:
    // когда у переменных и полей одинаковые имена
    _User {
        email,
        user_name,
        active: true,
        sign_in_count: 1,
    }
}

fn _test1(data: &_User) {
    println!("\n[test1] Получил не изменяумую ссылку на структуру {}", data);
    println!("[test1] Значение поля полученной структуры по ссылке {}\n", data.user_name);
}


#[derive(Debug)]
struct _Rectangle {
    width: u32,
    height: u32,
}


// Определение методов для структуры Rectangle
// Каждая структура может иметь несколько блоков impl.
impl _Rectangle {

    fn _area(&self) -> u32 {
        self.width * self.height
    }

    // Реализация метода can_hold в структуре Rectangle, который берет
    // еще один экземпляр структуры Rectangle в качестве параметра
    fn _can_hold(&self, other: &_Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Связанные функции:
    // это функция которая не принимает ссылку на экземпляр структуры (self) но находится в контексте структуры.
    // Создание квадратной структуры Rectangle вместо того, чтобы указывать одно и то же значение дважды:
    fn _square(size: u32) -> _Rectangle {
        _Rectangle { width: size, height: size }
    }
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


pub fn _start_rectangles_struct() {
    println!("\n================================================================================");
    // Расчет площади прямоугольника, заданной отдельными переменными
    // ширины width и высоты height
    let rect1 = _Rectangle { width: 30, height: 50 };
    let rect2 = _Rectangle { width: 10, height: 40 };
    let rect3 = _Rectangle { width: 60, height: 45 };

    println!("rect1 равен {:#?}", rect1);
    // println!("Площадь прямоугольника равна {} квадратным пикселам.", area(&rect1));
    println!("Площадь прямоугольника равна {} квадратным пикселам.", rect1._area());

    println!("Может ли rect1 содержать в себе rect2? {}", rect1._can_hold(&rect2));
    println!("Может ли rect1 содержать в себе rect3? {}", rect1._can_hold(&rect3));

    // Синтаксис вызова связных функций < :: >
    let square1 = _Rectangle::_square(10);
    println!("Полученный квадрат из связной функции структуры <Rectangle> square1 = {:#?}", square1);

}




pub fn _start_example_struct() {
    // Структура, или struct, — это настраиваемый тип данных, который позволяет вам
    // именовать и упаковывать вместе несколько связанных значений, составляющих смысловую группу.

    // Создание экземпляра структуры User:
    let user1: _User = _User {
        user_name: String::from("Pasha"),
        email: String::from("pasha@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("\n[user1] Экземпляр структуры: {}\n", user1);

    // Изменение значения в поле email экземпляра структуры User:
    let mut user2: _User = _User {
        user_name: String::from("Masha"),
        email: String::from("masha@gmail.com"),
        sign_in_count: 2,
        active: true,
    };

    user2.user_name = String::from("Masha Sexy!");
    println!("[user2] Экземпляр структуры: {}\n", user2);

    let user3: _User = _build_user(String::from("Sasha"), String::from("sasha@gmail.com"));
    println!("[user3] Экземпляр структуры: {}\n", user3);

    // Создание нового экземпляра структуры User с использованием
    // некоторых значений из user1
    let user4: _User = _User {
        user_name: String::from("Alex"),
        email: String::from("alex@gmail.com"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("[user4] Экземпляр структуры: {}\n", user4);

    // Синтаксис .. указывает
    // на то, что остальные поля, не заданные явно, должны иметь то же значение, что
    // и поля в конкретном экземпляре.
    let user5: _User = _User {
        user_name: String::from("Katy"),
        email: String::from("katy@gmail.com"),
        ..user1
    };
    _test1(&user5);
    println!("[user5] Экземпляр структуры: {}\n", user5);


    // Использование кортежных структур без именованных
    // полей для создания разных типов:
    // #[derive(Debug)]
    // struct Color(i32, i32, i32);
    // #[derive(Debug)]
    // struct Point(i32, i32, i32);

    // let black: Color = Color(0, 0, 0);
    // let origin: Point = Point(12, 33, 55);
    // println!("Кортежные структуры [black] и [origin]: {:?}, {:?}", black, origin);

}
