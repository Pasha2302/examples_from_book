## Установка инструмента Rustup и настройка проекта:
Чтобы установить инструмент `rustup`, который автоматически установит последнюю стабильную версию языка Rust,
выполните следующую команду в вашем терминале:

```bash:
    $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

###  Использование команды cargo search для поиска версий нужной библиотеки:
Вы можете использовать команду cargo search,
для поиска последней версии библиотеки (например библиотеки ненепации случайных чисел <rand>)
непосредственно из командной строки:

```bash
cargo search rand
```
Если библиотека уже указана в вашем Cargo.toml,
вы можете использовать команду cargo update для обновления всех зависимостей до их последних версий.
Это обновит ваши зависимости до последних версий, которые соответствуют указанным в Cargo.toml диапазонам версий.

```bash
cargo update
```

Если вам нужно проверить доступные версии,
вы можете использовать команду cargo tree для просмотра текущих версий всех зависимостей:

```bash
cargo tree
```


### Варианты инициализации проекта:
Создадим новый проект с использованием Cargo (создает дерикторию проекта и все нужные файлы):

```bash
cargo new nameProject
```
Иницилизировать новый проект в уже созданной директории (вмето cargo new nameProject):

```bash
cargo init
```


## Дополнительные команды
Git — это распространенная система управления версиями. С помощью флага --vcs вы
можете изменить команду cargo new, чтобы использовать другую систему управления
версиями или вообще не использовать никакой системы. Выполните команду cargo new

```bash
--help, чтобы увидеть имеющиеся варианты.
```

Эта команда создает исполняемый файл, правда, не в вашем текущем каталоге, а в target/debug/hello_cargo
Выпол­нение команды cargo build в первый раз также приводит к тому, 
что Cargo созда­ет новый файл на верхнем уровне — Cargo.lock. Этот файл отслеживает точные
версии зависимостей в проекте.
Вам никогда не придется изменять этот файл вручную, Cargo управляет его содержимым за вас.

```bash
cargo build   (Выполняем внутри проекта)
```

Также можем исполь­зовать команду cargo run для компиляции кода и последующего выполнения ре­
зультирующего исполняемого файла, и все это в одной команде:

```bash
cargo run   (Выполняем внутри проекта)
```

Cargo также предоставляет команду cargo check. Эта команда быстро проверяет
ваш код, чтобы убедиться в его компилируемости, но не создает исполняемый файл:

```bash
cargo check  (Выполняем внутри проекта)
```

Когда ваш проект будет окончательно готов к релизу, вы можете использовать
­команду cargo build --release для его компиляции с оптимизациями. Эта коман­
да создаст исполняемый файл в target/release вместо target/debug. Оптимизации
ускорят работу кода Rust, но их включение увеличивает время, необходимое для
компиляции программы. Вот почему существуют два разных профиля:

```bash
cargo build --release    (Выполняем внутри проекта)
```

В случае с простыми проектами у Cargo нет какой-то большой ценности по срав­
нению с компилятором rustc

```bash
rustc nameFile.rs   (Компиляция исходного кода вне проекта.)
```

Еще одно приятное свойство Cargo заключается в том, что вы можете
выполнить команду cargo doc --open, которая выведет документацию, порождаемую
всеми вашими зависимостями, локально и откроет ее в браузере. К примеру, если вас
интересует прочая функциональность упаковки rand, то выполните cargo doc --open
и нажмите rand на боковой панели слева.

```bash
cargo doc --open   (Выполняем внутри проекта)
```
