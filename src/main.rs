use std::{fs, path::Path, process};
use serde::Deserialize;


fn main() {
	let config = init();
	let port = if config.port.is_none() {80} else {config.port.unwrap()};
	println!("Сервер запущен на: 127.0.0.1:{}", port);
}

// 1.2 загруза базы данных пользователей

// 2. запус многопотока
// 2.1 запуск функции приёма
// 2.2 запуск таймеров и тд.

// 3. обработка запросов (исключение неверных запросов)
// 3.1 разбиваем запрос на начало и сырые данные
// 3.2 разбиваем начало на путь и заголовки

fn erorr(text: &str) -> ! {
	println!("{}", text);
	process::exit(0);
}

const CONFIG: &str = "config.toml";
#[derive(Deserialize)]
struct Config {
	port: Option<u16>
}

// 1. процесс иницилязии
fn init() -> Config {
	// 1.1 загрузка файла настроек
	let path = Path::new(CONFIG);
	if !path.is_file() {
		erorr(&format!("Нет файла {} или он не является файлом", CONFIG))};
	let Ok(content) = fs::read_to_string(path) else {
		erorr(&format!("Не удалось прочитать файл {}", CONFIG))};
	match toml::from_str::<Config>(&content) {
		Ok(v) => v, Err(e) =>
		erorr(&format!("Не удалось распарсить файл {}: {}", CONFIG, e))
	}
}