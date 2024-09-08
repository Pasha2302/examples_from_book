mod modules; // Подключаем модуль `modules`, определённый в `modules.rs`

use modules::branches_if_else; // Импортируем модуль `branches_if_else` из `modules`
use modules::guessing_game;
use modules::iterator_implementation;
use modules::variables;

use modules::request_example;
use modules::using_structures;
use modules::enumeration;


fn _test_func(use_func: &str) {
    if use_func == "enum" {
        enumeration::_start_example_enum();

    } else {
        iterator_implementation::_run_iterator();
        branches_if_else::_elseif_example();
        variables::_var_example();
        guessing_game::_start_game();

        using_structures::_start_example_struct();
    }
}


fn _start_get_data() {
    match request_example::_get_data() {
        Ok(()) => println!("Operation successful"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}


fn main() {
    
    _test_func("enum");

}
