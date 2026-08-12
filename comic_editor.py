

### 1. `comic_editor.py` (Python)

```python
# comic_editor.py — Python версия

import json
import os
from datetime import datetime
from colorama import init, Fore, Style

init(autoreset=True)

class Character:
    def __init__(self, name, emojis, description=""):
        self.name = name
        self.emojis = emojis  # dict: emotion -> ascii art
        self.description = description

    def get_ascii(self, emotion="default"):
        return self.emojis.get(emotion, self.emojis.get("default", "👤"))

class ComicEditor:
    def __init__(self):
        self.characters = self._init_characters()
        self.scene = []
        self.background = ""

    def _init_characters(self):
        return {
            "hero": Character(
                "Hero",
                {
                    "default": "(^_^)\n /|\\\n / \\",
                    "радость": "(^_^)\n /|\\\n / \\",
                    "грусть": "(T_T)\n /|\\\n / \\",
                    "злость": "(>_<)\n /|\\\n / \\",
                    "удивление": "(O_O)\n /|\\\n / \\",
                    "страх": "(>_<)\n /|\\\n / \\"
                },
                "Отважный герой"
            ),
            "villain": Character(
                "Villain",
                {
                    "default": "(-_-)\n /|\\\n / \\",
                    "радость": "(^_^)\n /|\\\n / \\",
                    "грусть": "(T_T)\n /|\\\n / \\",
                    "злость": "(>_<)\n /|\\\n / \\",
                    "удивление": "(O_O)\n /|\\\n / \\",
                    "страх": "(>_<)\n /|\\\n / \\"
                },
                "Коварный злодей"
            ),
            "robot": Character(
                "Robot",
                {
                    "default": "[0_0]\n /|\\\n / \\",
                    "радость": "[^_^]\n /|\\\n / \\",
                    "грусть": "[T_T]\n /|\\\n / \\",
                    "злость": "[>_<]\n /|\\\n / \\",
                    "удивление": "[O_O]\n /|\\\n / \\",
                    "страх": "[>_<]\n /|\\\n / \\"
                },
                "Механический робот"
            ),
            "monster": Character(
                "Monster",
                {
                    "default": "({0_0})\n /|\\\n / \\",
                    "радость": "({^_^})\n /|\\\n / \\",
                    "грусть": "({T_T})\n /|\\\n / \\",
                    "злость": "({>_<})\n /|\\\n / \\",
                    "удивление": "({O_O})\n /|\\\n / \\",
                    "страх": "({>_<})\n /|\\\n / \\"
                },
                "Страшный монстр"
            ),
            "princess": Character(
                "Princess",
                {
                    "default": "(✿◠‿◠)\n /|\\\n / \\",
                    "радость": "(✿◠‿◠)\n /|\\\n / \\",
                    "грусть": "(✿T_T)\n /|\\\n / \\",
                    "злость": "(✿>_<)\n /|\\\n / \\",
                    "удивление": "(✿O_O)\n /|\\\n / \\",
                    "страх": "(✿>_<)\n /|\\\n / \\"
                },
                "Прекрасная принцесса"
            ),
            "knight": Character(
                "Knight",
                {
                    "default": "[⚔️] \n /|\\\n / \\",
                    "радость": "[⚔️^_^]\n /|\\\n / \\",
                    "грусть": "[⚔️T_T]\n /|\\\n / \\",
                    "злость": "[⚔️>_<]\n /|\\\n / \\",
                    "удивление": "[⚔️O_O]\n /|\\\n / \\",
                    "страх": "[⚔️>_<]\n /|\\\n / \\"
                },
                "Доблестный рыцарь"
            )
        }

    def list_characters(self):
        print(Fore.CYAN + "Доступные персонажи:")
        for i, (key, char) in enumerate(self.characters.items(), 1):
            print(f"  {i}. {char.name} — {char.description}")

    def get_character(self, choice):
        keys = list(self.characters.keys())
        if 1 <= choice <= len(keys):
            return keys[choice-1]
        return None

    def create_scene(self):
        print(Fore.CYAN + "🎭 Создание сцены комикса")
        self.list_characters()

        while True:
            try:
                choice = int(input("Выберите персонажа (0 — завершить): "))
                if choice == 0:
                    break
                char_key = self.get_character(choice)
                if not char_key:
                    print(Fore.RED + "❌ Неверный выбор.")
                    continue

                char = self.characters[char_key]
                print(f"Доступные эмоции: {', '.join(char.emojis.keys())}")
                emotion = input("Выберите эмоцию: ").strip().lower()
                if emotion not in char.emojis:
                    emotion = "default"

                dialogue = input("Введите реплику персонажа: ")

                self.scene.append({
                    "character": char_key,
                    "name": char.name,
                    "emotion": emotion,
                    "dialogue": dialogue,
                    "ascii": char.get_ascii(emotion)
                })
                print(Fore.GREEN + f"✅ {char.name} добавлен в сцену!")

            except ValueError:
                print(Fore.RED + "❌ Введите число.")

    def render_scene(self):
        if not self.scene:
            print(Fore.YELLOW + "Сцена пуста. Добавьте персонажей.")
            return

        width = 50
        print("\n" + "+" + "-" * (width - 2) + "+")
        for item in self.scene:
            lines = item['ascii'].split('\n')
            for line in lines:
                print(f"| {line:<{width-4}} |")
            if item['dialogue']:
                print(f"| {Fore.YELLOW}💬 {item['dialogue']:<{width-6}}{Fore.RESET} |")
            print("|" + "-" * (width - 2) + "|")
        print("+" + "-" * (width - 2) + "+")

    def save_html(self, filename="comic.html"):
        if not self.scene:
            print(Fore.YELLOW + "Нет сцены для сохранения.")
            return

        html = f"""<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>Мой комикс</title>
<style>
body {{ font-family: monospace; background: #f0f0f0; padding: 20px; }}
.panel {{ background: white; border: 2px solid #333; border-radius: 10px; padding: 20px; margin: 20px auto; max-width: 600px; box-shadow: 0 4px 8px rgba(0,0,0,0.1); }}
.character {{ white-space: pre; font-size: 14px; line-height: 1.2; text-align: center; }}
.dialogue {{ font-size: 18px; color: #2c3e50; text-align: center; margin: 10px 0; padding: 10px; background: #ecf0f1; border-radius: 10px; }}
.name {{ font-weight: bold; color: #2980b9; text-align: center; }}
</style>
</head>
<body>
<h1 style="text-align:center;">🎭 Мой комикс</h1>
"""
        for item in self.scene:
            html += f"""
<div class="panel">
    <div class="name">{item['name']} ({item['emotion']})</div>
    <div class="character">{item['ascii']}</div>
    <div class="dialogue">💬 {item['dialogue']}</div>
</div>
"""
        html += """
</body>
</html>
"""
        with open(filename, 'w', encoding='utf-8') as f:
            f.write(html)
        print(Fore.GREEN + f"💾 Комикс сохранён в {filename}")

    def save_json(self, filename="comic_project.json"):
        data = {
            "timestamp": datetime.now().isoformat(),
            "scene": self.scene
        }
        with open(filename, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        print(Fore.GREEN + f"💾 Проект сохранён в {filename}")

    def load_json(self, filename="comic_project.json"):
        if not os.path.exists(filename):
            print(Fore.RED + f"❌ Файл {filename} не найден.")
            return
        with open(filename, 'r', encoding='utf-8') as f:
            data = json.load(f)
        self.scene = data.get("scene", [])
        print(Fore.GREEN + f"✅ Проект загружен из {filename}")

def main():
    editor = ComicEditor()
    while True:
        print(Fore.CYAN + "\n🎭 Comic Editor Pro (Python)")
        print("1. Создать новую сцену")
        print("2. Показать сцену")
        print("3. Сохранить как HTML")
        print("4. Сохранить проект (JSON)")
        print("5. Загрузить проект (JSON)")
        print("6. Выход")
        choice = input("Выберите действие: ").strip()

        if choice == "1":
            editor.create_scene()
        elif choice == "2":
            editor.render_scene()
        elif choice == "3":
            editor.save_html()
        elif choice == "4":
            editor.save_json()
        elif choice == "5":
            editor.load_json()
        elif choice == "6":
            print("До свидания!")
            break
        else:
            print(Fore.RED + "❌ Неверный выбор.")

if __name__ == "__main__":
    main()
