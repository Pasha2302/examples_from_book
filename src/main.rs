mod modules;  // Подключаем модуль `modules`, определённый в `modules.rs`

use modules::branches_if_else;  // Импортируем модуль `mod1` из `modules`
use modules::variables;  // Импортируем модуль `mod2` из `modules`


fn main() {
    println!("\n\nHello, world!\n\n");

    branches_if_else::elseif_example();
    variables::var_example();
}
