# ⚡ Ultra-Fast Telegram Like Bot (Rust) 🦀

![Rust Version](https://img.shields.io/badge/Rust-1.70+-orange)
![License](https://img.shields.io/badge/License-MIT-blue)
![Performance](https://img.shields.io/badge/Speed-20ms-brightgreen)

Этот проект представляет собой высокоскоростного юзер-бота для Telegram, способного ставить лайки в приватных чатах с рекордной скоростью до 20 мс. Хотя проект не был выпущен в продакшн, я публикую его код для демонстрации возможностей Rust в создании высокопроизводительных Telegram-клиентов.

## 🚀 Установка и запуск

### 🔧 Требования
- Rust 1.70 или новее
- Telegram API ID и Hash (получить на [my.telegram.org](https://my.telegram.org))
- Номер телефона аккаунта Telegram (в международном формате)

### 📥 Установка на сервер (с root-доступом)

```bash
# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Создание проекта (Пути используются, как пример)
cd /home/
cargo new bot-likedws
cd /home/bot-likedws/

# Настройка окружения
nano .env

```
### После проведенных выше описанных мероприятий, подгружаете файлы из репозитория по следующим путям:
- src -> /home/bot-likedws/
- Cargo.toml -> /home/bot-likedws/

### 📥 Настройка .env (с root-доступом)
- TELEGRAM_PHONE="+XXXXXXXXXXX" 
- TELEGRAM_API_ID=1234567
- TELEGRAM_API_HASH="0123456789abcdef"

### 📥 Сборка и запуск

```bash
cargo build --release
cargo run --release
```

- Если вы хотите использовать данного бота не для теста, рекомендую использовать screen или tmux.

```bash
cargo build --release
sudo apt install -y tmux
tmux new -s bot-session
cargo run --release

#Вводите код
#ctrl-b, D
```

### 📊 Производительность
| Название                | Описание                                                        |
|-------------------------|-----------------------------------------------------------------|
| Среднее время отклика   | 18-22мс (при учете сервера в Амстердаме.)                       |
| Макс. нагрузка          | 1500 RPS                                                        |
| Потребление RAM         | ~150 MB                                                         |


### ⚠️ Ограничения

    Требуется валидный номер Telegram

    API ключи должны быть получены через my.telegram.org

    Для работы в Китае/Иране нужен прокси


