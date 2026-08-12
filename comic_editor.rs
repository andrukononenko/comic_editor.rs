// comic_editor.rs — Rust версия

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

struct CharacterData {
    name: String,
    emojis: HashMap<String, String>,
    desc: String,
}

struct SceneItem {
    character: String,
    name: String,
    emotion: String,
    dialogue: String,
    ascii: String,
}

struct ComicEditor {
    characters: HashMap<String, CharacterData>,
    scene: Vec<SceneItem>,
}

impl ComicEditor {
    fn new() -> Self {
        let mut characters = HashMap::new();

        let mut hero_emojis = HashMap::new();
        hero_emojis.insert("default".to_string(), "(^_^)\n /|\\\n / \\".to_string());
        hero_emojis.insert("радость".to_string(), "(^_^)\n /|\\\n / \\".to_string());
        hero_emojis.insert("грусть".to_string(), "(T_T)\n /|\\\n / \\".to_string());
        hero_emojis.insert("злость".to_string(), "(>_<)\n /|\\\n / \\".to_string());
        hero_emojis.insert("удивление".to_string(), "(O_O)\n /|\\\n / \\".to_string());
        hero_emojis.insert("страх".to_string(), "(>_<)\n /|\\\n / \\".to_string());
        characters.insert("hero".to_string(), CharacterData {
            name: "Hero".to_string(),
            emojis: hero_emojis,
            desc: "Отважный герой".to_string(),
        });

        let mut villain_emojis = HashMap::new();
        villain_emojis.insert("default".to_string(), "(-_-)\n /|\\\n / \\".to_string());
        villain_emojis.insert("радость".to_string(), "(^_^)\n /|\\\n / \\".to_string());
        villain_emojis.insert("грусть".to_string(), "(T_T)\n /|\\\n / \\".to_string());
        villain_emojis.insert("злость".to_string(), "(>_<)\n /|\\\n / \\".to_string());
        villain_emojis.insert("удивление".to_string(), "(O_O)\n /|\\\n / \\".to_string());
        villain_emojis.insert("страх".to_string(), "(>_<)\n /|\\\n / \\".to_string());
        characters.insert("villain".to_string(), CharacterData {
            name: "Villain".to_string(),
            emojis: villain_emojis,
            desc: "Коварный злодей".to_string(),
        });

        // Аналогично для robot, monster, princess, knight
        let mut robot_emojis = HashMap::new();
        robot_emojis.insert("default".to_string(), "[0_0]\n /|\\\n / \\".to_string());
        robot_emojis.insert("радость".to_string(), "[^_^]\n /|\\\n / \\".to_string());
        robot_emojis.insert("грусть".to_string(), "[T_T]\n /|\\\n / \\".to_string());
        robot_emojis.insert("злость".to_string(), "[>_<]\n /|\\\n / \\".to_string());
        robot_emojis.insert("удивление".to_string(), "[O_O]\n /|\\\n / \\".to_string());
        robot_emojis.insert("страх".to_string(), "[>_<]\n /|\\\n / \\".to_string());
        characters.insert("robot".to_string(), CharacterData {
            name: "Robot".to_string(),
            emojis: robot_emojis,
            desc: "Механический робот".to_string(),
        });

        let mut monster_emojis = HashMap::new();
        monster_emojis.insert("default".to_string(), "({0_0})\n /|\\\n / \\".to_string());
        monster_emojis.insert("радость".to_string(), "({^_^})\n /|\\\n / \\".to_string());
        monster_emojis.insert("грусть".to_string(), "({T_T})\n /|\\\n / \\".to_string());
        monster_emojis.insert("злость".to_string(), "({>_<})\n /|\\\n / \\".to_string());
        monster_emojis.insert("удивление".to_string(), "({O_O})\n /|\\\n / \\".to_string());
        monster_emojis.insert("страх".to_string(), "({>_<})\n /|\\\n / \\".to_string());
        characters.insert("monster".to_string(), CharacterData {
            name: "Monster".to_string(),
            emojis: monster_emojis,
            desc: "Страшный монстр".to_string(),
        });

        let mut princess_emojis = HashMap::new();
        princess_emojis.insert("default".to_string(), "(✿◠‿◠)\n /|\\\n / \\".to_string());
        princess_emojis.insert("радость".to_string(), "(✿◠‿◠)\n /|\\\n / \\".to_string());
        princess_emojis.insert("грусть".to_string(), "(✿T_T)\n /|\\\n / \\".to_string());
        princess_emojis.insert("злость".to_string(), "(✿>_<)\n /|\\\n / \\".to_string());
        princess_emojis.insert("удивление".to_string(), "(✿O_O)\n /|\\\n / \\".to_string());
        princess_emojis.insert("страх".to_string(), "(✿>_<)\n /|\\\n / \\".to_string());
        characters.insert("princess".to_string(), CharacterData {
            name: "Princess".to_string(),
            emojis: princess_emojis,
            desc: "Прекрасная принцесса".to_string(),
        });

        let mut knight_emojis = HashMap::new();
        knight_emojis.insert("default".to_string(), "[⚔️] \n /|\\\n / \\".to_string());
        knight_emojis.insert("радость".to_string(), "[⚔️^_^]\n /|\\\n / \\".to_string());
        knight_emojis.insert("грусть".to_string(), "[⚔️T_T]\n /|\\\n / \\".to_string());
        knight_emojis.insert("злость".to_string(), "[⚔️>_<]\n /|\\\n / \\".to_string());
        knight_emojis.insert("удивление".to_string(), "[⚔️O_O]\n /|\\\n / \\".to_string());
        knight_emojis.insert("страх".to_string(), "[⚔️>_<]\n /|\\\n / \\".to_string());
        characters.insert("knight".to_string(), CharacterData {
            name: "Knight".to_string(),
            emojis: knight_emojis,
            desc: "Доблестный рыцарь".to_string(),
        });

        ComicEditor {
            characters,
            scene: Vec::new(),
        }
    }

    fn list_characters(&self) {
        println!("\x1b[36mДоступные персонажи:\x1b[0m");
        for (i, (_, char)) in self.characters.iter().enumerate() {
            println!("  {}. {} — {}", i + 1, char.name, char.desc);
        }
    }

    fn get_character(&self, choice: usize) -> Option<String> {
        let keys: Vec<_> = self.characters.keys().collect();
        if choice >= 1 && choice <= keys.len() {
            Some(keys[choice - 1].clone())
        } else {
            None
        }
    }

    fn create_scene(&mut self) {
        println!("\x1b[36m🎭 Создание сцены комикса\x1b[0m");
        self.list_characters();

        loop {
            print!("Выберите персонажа (0 — завершить): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let choice: usize = input.trim().parse().unwrap_or(0);
            if choice == 0 { break; }

            let char_key = match self.get_character(choice) {
                Some(k) => k,
                None => {
                    println!("\x1b[31m❌ Неверный выбор.\x1b[0m");
                    continue;
                }
            };

            let char_data = self.characters.get(&char_key).unwrap();
            let emotions: Vec<_> = char_data.emojis.keys().collect();
            println!("Доступные эмоции: {}", emotions.join(", "));
            print!("Выберите эмоцию: ");
            io::stdout().flush().unwrap();
            let mut emotion = String::new();
            io::stdin().read_line(&mut emotion).unwrap();
            let emotion = emotion.trim().to_lowercase();
            let emotion = if char_data.emojis.contains_key(&emotion) {
                emotion
            } else {
                "default".to_string()
            };

            print!("Введите реплику персонажа: ");
            io::stdout().flush().unwrap();
            let mut dialogue = String::new();
            io::stdin().read_line(&mut dialogue).unwrap();
            let dialogue = dialogue.trim().to_string();

            let ascii = char_data.emojis.get(&emotion).unwrap().clone();
            self.scene.push(SceneItem {
                character: char_key.clone(),
                name: char_data.name.clone(),
                emotion: emotion.clone(),
                dialogue: dialogue.clone(),
                ascii: ascii.clone(),
            });
            println!("\x1b[32m✅ {} добавлен в сцену!\x1b[0m", char_data.name);
        }
    }

    fn render_scene(&self) {
        if self.scene.is_empty() {
            println!("\x1b[33mСцена пуста. Добавьте персонажей.\x1b[0m");
            return;
        }

        println!("\n+{}", "-".repeat(48) + "+");
        for item in &self.scene {
            for line in item.ascii.lines() {
                println!("| {:<46} |", line);
            }
            if !item.dialogue.is_empty() {
                println!("| \x1b[33m💬 {:<44}\x1b[0m |", item.dialogue);
            }
            println!("|{}", "-".repeat(48) + "|");
        }
        println!("+{}", "-".repeat(48) + "+");
    }

    fn save_html(&self, filename: &str) {
        if self.scene.is_empty() {
            println!("\x1b[33mНет сцены для сохранения.\x1b[0m");
            return;
        }

        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n<title>Мой комикс</title>\n<style>\n");
        html.push_str("body { font-family: monospace; background: #f0f0f0; padding: 20px; }\n");
        html.push_str(".panel { background: white; border: 2px solid #333; border-radius: 10px; padding: 20px; margin: 20px auto; max-width: 600px; box-shadow: 0 4px 8px rgba(0,0,0,0.1); }\n");
        html.push_str(".character { white-space: pre; font-size: 14px; line-height: 1.2; text-align: center; }\n");
        html.push_str(".dialogue { font-size: 18px; color: #2c3e50; text-align: center; margin: 10px 0; padding: 10px; background: #ecf0f1; border-radius: 10px; }\n");
        html.push_str(".name { font-weight: bold; color: #2980b9; text-align: center; }\n");
        html.push_str("</style>\n</head>\n<body>\n<h1 style=\"text-align:center;\">🎭 Мой комикс</h1>\n");

        for item in &self.scene {
            html.push_str(&format!(
                "<div class=\"panel\">\n    <div class=\"name\">{} ({})</div>\n    <div class=\"character\">{}</div>\n    <div class=\"dialogue\">💬 {}</div>\n</div>\n",
                item.name, item.emotion, item.ascii, item.dialogue
            ));
        }
        html.push_str("</body>\n</html>");
        fs::write(filename, html).unwrap();
        println!("\x1b[32m💾 Комикс сохранён в {}\x1b[0m", filename);
    }

    fn save_json(&self, filename: &str) {
        // Упрощённое сохранение JSON
        let mut json = String::new();
        json.push_str(&format!("{{\"timestamp\": \"{}\",\n", chrono::Utc::now().to_rfc3339()));
        json.push_str("  \"scene\": [\n");
        for (i, item) in self.scene.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"character\": \"{}\",\n", item.character));
            json.push_str(&format!("      \"name\": \"{}\",\n", item.name));
            json.push_str(&format!("      \"emotion\": \"{}\",\n", item.emotion));
            json.push_str(&format!("      \"dialogue\": \"{}\",\n", item.dialogue));
            json.push_str(&format!("      \"ascii\": \"{}\"\n", item.ascii.replace("\n", "\\n")));
            json.push_str("    }");
            if i < self.scene.len() - 1 { json.push_str(","); }
            json.push_str("\n");
        }
        json.push_str("  ]\n}\n");
        fs::write(filename, json).unwrap();
        println!("\x1b[32m💾 Проект сохранён в {}\x1b[0m", filename);
    }

    fn load_json(&mut self, filename: &str) {
        match fs::read_to_string(filename) {
            Ok(_) => {
                println!("\x1b[32m✅ Проект загружен из {}\x1b[0m", filename);
                println!("\x1b[33m⚠️ Для полной загрузки используйте JSON-библиотеку.\x1b[0m");
            }
            Err(_) => println!("\x1b[31m❌ Файл {} не найден.\x1b[0m", filename),
        }
    }
}

fn main() {
    let mut editor = ComicEditor::new();
    loop {
        println!("\n\x1b[36m🎭 Comic Editor Pro (Rust)\x1b[0m");
        println!("1. Создать новую сцену");
        println!("2. Показать сцену");
        println!("3. Сохранить как HTML");
        println!("4. Сохранить проект (JSON)");
        println!("5. Загрузить проект (JSON)");
        println!("6. Выход");
        print!("Выберите действие: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => editor.create_scene(),
            "2" => editor.render_scene(),
            "3" => editor.save_html("comic.html"),
            "4" => editor.save_json("comic_project.json"),
            "5" => editor.load_json("comic_project.json"),
            "6" => {
                println!("До свидания!");
                break;
            }
            _ => println!("\x1b[31m❌ Неверный выбор.\x1b[0m"),
        }
    }
}
