mod modules;  // Подключаем модуль `modules`, определённый в `modules.rs`

use modules::branches_if_else;  // Импортируем модуль `mod1` из `modules`
use modules::variables;  // Импортируем модуль `mod2` из `modules`
use modules::iterator_implementation;
use modules::guessing_game;

use modules::request_example;


fn _test_func() {
    iterator_implementation::_run_iterator();
    branches_if_else::_elseif_example();
    variables::_var_example();
}


fn _start_get_data() {
    match request_example::_get_data() {
        Ok(()) => println!("Operation successful"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}


fn main() {
    println!("\n\nHello, world!\n\n");

    guessing_game::_start_game();

    // _start_get_data();
    
}
