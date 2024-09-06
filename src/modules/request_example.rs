use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}; // Импортируем нужные типы заголовков
use reqwest::{Client, multipart}; // Импортируем клиента и поддержку multipart-форм
use reqwest::cookie::Jar; // Импортируем тип для работы с cookies
use serde_json::json; // Импортируем макрос json из serde_json для работы с JSON
use std::sync::Arc; // Импортируем Arc для многопоточного использования cookies
use std::fs; // Для работы с файловой системой
use tokio; // Используем асинхронное программирование с Tokio


#[tokio::main]
pub async fn _get_data() -> Result<(), Box<dyn std::error::Error>> {

    // ---------------------------
    // 1. Подготовка заголовков
    // ---------------------------
    let mut headers: HeaderMap = HeaderMap::new(); // Создаем карту заголовков
    headers.insert(USER_AGENT, HeaderValue::from_static("My Rust Client 1.0")); // Устанавливаем User-Agent
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json")); // Устанавливаем Content-Type
    headers.insert("Custom-Header", HeaderValue::from_static("CustomValue")); // Добавляем пользовательский заголовок
    headers.insert("Authorization", HeaderValue::from_static("8f5d3f9e-2d7b-4b8f-a7c1-9c4e8b12a6e3"));

    // ---------------------------
    // 2. Параметры запроса (Query Parameters)
    // ---------------------------
    let params: [(&str, &str); 2] = [("param1", "value1"), ("param2", "value2")]; // Определяем параметры запроса

    // ---------------------------
    // 3. Данные JSON для тела запроса
    // ---------------------------
    let json_data: serde_json::Value = json!({
        "username": "test_user",
        "email": "test@example.com"
    }); // Создаем JSON-объект

    // ---------------------------
    // 4. Multipart Form для передачи файлов
    // ---------------------------
    // Читаем содержимое файла и создаем часть для multipart-запроса
    let file_content: Vec<u8> = fs::read("./file.txt")?; // Прочитайте файл (укажите правильный путь)
    let part: multipart::Part = multipart::Part::bytes(file_content).file_name("file.txt"); // Создаем часть с содержимым файла

    let form: multipart::Form = multipart::Form::new()
        .text("description", "File description") // Добавляем текстовое поле в форму
        .part("file_field", part); // Добавляем файл как часть формы

    // ---------------------------
    // 5. Cookies
    // ---------------------------
    let jar: Arc<Jar> = Arc::new(Jar::default()); // Создаем хранилище cookies
    jar.add_cookie_str("session_id=123456", &"https://httpbin.org".parse()?); // Добавляем cookie

    // Создаем HTTP клиент с поддержкой cookies
    let client: Client = Client::builder()
        .default_headers(headers) // Устанавливаем заголовки по умолчанию
        .cookie_store(true) // Включаем поддержку cookies
        .cookie_provider(jar) // Передаем cookies
        .build()?; // Собираем клиент

    // ---------------------------
    // 6. Отправка POST-запроса
    // ---------------------------
    let response: String = client
        .get("http://127.0.0.1:8000/api/v1/get-data-bonus/8/") // URL для запроса
        .query(&params) // Передаем параметры запроса
        .json(&json_data) // Передаем JSON-данные в теле запроса
        .multipart(form) // Передаем форму для загрузки файлов
        .send() // Отправляем запрос
        .await? // Ожидаем завершения выполнения асинхронного запроса
        .text() // Получаем ответ как текст
        .await?;

    // ---------------------------
    // 7. Вывод результата
    // ---------------------------
    println!("Response: {}", response); // Печатаем ответ от сервера

    Ok(())
}
