# COPRO-SNAKE 🐍

*A classic Snake game implementation in Rust with modern graphics and unique gameplay mechanics*

![Game Screenshot](./Screenshot.png)

## English 🇺🇸

### 🎮 About the Game

COPRO-SNAKE is a modern take on the classic Snake game, written in Rust using the Piston game engine with OpenGL rendering. The game features unique mechanics including health points, multiple item types, and advanced collision detection.

### ✨ Key Features

- **Health System**: Snake has HP (Health Points) - start with 3 HP
- **Multiple Item Types**:
  - 🟤 **Poop (Brown)**: Regular food for growth (+4 segments per item)
  - 🔴 **Heal (Red)**: Restores health (+1 HP)
  - 🟢 **Poison (Green)**: Damages the snake (-1 HP)
- **Advanced Mechanics**:
  - Teleportation when hitting screen boundaries
  - Invulnerability period for newly spawned segments
  - Dynamic item spawning with configurable drop rates
  - Score tracking system
  - Smooth movement with configurable FPS (30 FPS)

### 🚀 Installation & Setup

#### Prerequisites
- Install Rust from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- OpenGL support (usually pre-installed on most systems)

#### Building & Running
```bash
# Clone the repository
git clone <repository-url>
cd copro-snake

# Build the project
cargo build --release

# Run the game
cargo run --release
```

### 🎯 Controls

| Key | Action |
|-----|--------|
| ↑↓←→ | Move the snake |
| `Space` | Spawn new snake (restart) |
| `Enter` | Full game restart |
| `Esc` | Exit game |

### 🎲 Gameplay Mechanics

#### Scoring System
- Gain points by collecting items
- Score is displayed in real-time

#### Health System
- Start with 3 HP
- Lose HP by eating poison (-1 HP)
- Gain HP by eating heal items (+1 HP)
- Game over when HP reaches 0

#### Item Drop Rates
- **Poison**: 20% chance
- **Heal**: 7% chance  
- **Poop**: 73% chance (remaining probability)

#### Growth Mechanics
- Snake grows by 4 segments per poop consumed
- New segments have temporary invulnerability
- Movement becomes faster as snake grows

### 🛠️ Technical Details

- **Language**: Rust
- **Game Engine**: Piston 0.53.0
- **Graphics**: OpenGL with piston2d-opengl_graphics
- **Window System**: Glutin
- **Font**: Press Start 2P (retro gaming font)
- **Window Size**: 400x400 pixels
- **Target FPS**: 30

### 📁 Project Structure
```
copro-snake/
├── src/
│   └── main.rs          # Main game logic
├── assets/
│   └── PressStart2P-Regular.ttf  # Game font
├── Cargo.toml           # Dependencies
├── Screenshot.png       # Game screenshot
└── readme.md           # This file
```

### 🧩 Dependencies
- `piston` - Game engine framework
- `piston2d-graphics` - 2D graphics primitives
- `pistoncore-glutin_window` - Window management
- `piston2d-opengl_graphics` - OpenGL rendering backend
- `rand` - Random number generation

---

## Русский 🇷🇺

### 🎮 О игре

COPRO-SNAKE — это современная версия классической игры "Змейка", написанная на языке Rust с использованием игрового движка Piston и рендеринга OpenGL. Игра включает уникальные механики, такие как система здоровья, различные типы предметов и продвинутое обнаружение столкновений.

### ✨ Основные возможности

- **Система здоровья**: У змейки есть HP (очки здоровья) - начинаете с 3 HP
- **Различные типы предметов**:
  - 🟤 **Какашка (Коричневый)**: Обычная еда для роста (+4 сегмента за предмет)
  - 🔴 **Лечение (Красный)**: Восстанавливает здоровье (+1 HP)
  - 🟢 **Яд (Зеленый)**: Наносит урон змейке (-1 HP)
- **Продвинутые механики**:
  - Телепортация при достижении границ экрана
  - Период неуязвимости для новых сегментов
  - Динамическое появление предметов с настраиваемыми шансами
  - Система подсчета очков
  - Плавное движение с настраиваемым FPS (30 FPS)

### 🚀 Установка и настройка

#### Требования
- Установите Rust с [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- Поддержка OpenGL (обычно предустановлена в большинстве систем)

#### Сборка и запуск
```bash
# Клонируйте репозиторий
git clone <repository-url>
cd copro-snake

# Соберите проект
cargo build --release

# Запустите игру
cargo run --release
```

### 🎯 Управление

| Клавиша | Действие |
|---------|----------|
| ↑↓←→ | Движение змейки |
| `Пробел` | Создать новую змейку (перезапуск) |
| `Enter` | Полный перезапуск игры |
| `Esc` | Выход из игры |

### 🎲 Игровые механики

#### Система очков
- Получайте очки за сбор предметов
- Счет отображается в реальном времени

#### Система здоровья
- Начинаете с 3 HP
- Теряете HP, съедая яд (-1 HP)
- Получаете HP, съедая лечебные предметы (+1 HP)
- Игра заканчивается, когда HP достигает 0

#### Шансы выпадения предметов
- **Яд**: 20% шанс
- **Лечение**: 7% шанс
- **Какашка**: 73% шанс (оставшаяся вероятность)

#### Механики роста
- Змейка растет на 4 сегмента за каждую съеденную какашку
- Новые сегменты имеют временную неуязвимость
- Движение ускоряется по мере роста змейки

### 🛠️ Технические детали

- **Язык**: Rust
- **Игровой движок**: Piston 0.53.0
- **Графика**: OpenGL с piston2d-opengl_graphics
- **Система окон**: Glutin
- **Шрифт**: Press Start 2P (ретро-игровой шрифт)
- **Размер окна**: 400x400 пикселей
- **Целевой FPS**: 30

### 📁 Структура проекта
```
copro-snake/
├── src/
│   └── main.rs          # Основная логика игры
├── assets/
│   └── PressStart2P-Regular.ttf  # Шрифт игры
├── Cargo.toml           # Зависимости
├── Screenshot.png       # Скриншот игры
└── readme.md           # Этот файл
```

### 🧩 Зависимости
- `piston` - Фреймворк игрового движка
- `piston2d-graphics` - 2D графические примитивы
- `pistoncore-glutin_window` - Управление окнами
- `piston2d-opengl_graphics` - Backend рендеринга OpenGL
- `rand` - Генерация случайных чисел

---

## 📄 License

This project is open source. See the source code for more details.

## 👨‍💻 Author

**Roman A. Nosov** - [roman@nosoff.info](mailto:roman@nosoff.info)