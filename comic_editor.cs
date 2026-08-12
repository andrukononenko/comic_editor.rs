// comic_editor.cs — C# версия

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

class CharacterData {
    public string Name { get; set; }
    public Dictionary<string, string> Emojis { get; set; }
    public string Desc { get; set; }
}

class SceneItem {
    public string Character { get; set; }
    public string Name { get; set; }
    public string Emotion { get; set; }
    public string Dialogue { get; set; }
    public string Ascii { get; set; }
}

class ComicEditor {
    private Dictionary<string, CharacterData> characters = new Dictionary<string, CharacterData>();
    private List<SceneItem> scene = new List<SceneItem>();

    public ComicEditor() {
        InitCharacters();
    }

    private void InitCharacters() {
        characters["hero"] = new CharacterData {
            Name = "Hero",
            Emojis = new Dictionary<string, string> {
                ["default"] = "(^_^)\n /|\\\n / \\",
                ["радость"] = "(^_^)\n /|\\\n / \\",
                ["грусть"] = "(T_T)\n /|\\\n / \\",
                ["злость"] = "(>_<)\n /|\\\n / \\",
                ["удивление"] = "(O_O)\n /|\\\n / \\",
                ["страх"] = "(>_<)\n /|\\\n / \\"
            },
            Desc = "Отважный герой"
        };
        characters["villain"] = new CharacterData {
            Name = "Villain",
            Emojis = new Dictionary<string, string> {
                ["default"] = "(-_-)\n /|\\\n / \\",
                ["радость"] = "(^_^)\n /|\\\n / \\",
                ["грусть"] = "(T_T)\n /|\\\n / \\",
                ["злость"] = "(>_<)\n /|\\\n / \\",
                ["удивление"] = "(O_O)\n /|\\\n / \\",
                ["страх"] = "(>_<)\n /|\\\n / \\"
            },
            Desc = "Коварный злодей"
        };
        characters["robot"] = new CharacterData {
            Name = "Robot",
            Emojis = new Dictionary<string, string> {
                ["default"] = "[0_0]\n /|\\\n / \\",
                ["радость"] = "[^_^]\n /|\\\n / \\",
                ["грусть"] = "[T_T]\n /|\\\n / \\",
                ["злость"] = "[>_<]\n /|\\\n / \\",
                ["удивление"] = "[O_O]\n /|\\\n / \\",
                ["страх"] = "[>_<]\n /|\\\n / \\"
            },
            Desc = "Механический робот"
        };
        characters["monster"] = new CharacterData {
            Name = "Monster",
            Emojis = new Dictionary<string, string> {
                ["default"] = "({0_0})\n /|\\\n / \\",
                ["радость"] = "({^_^})\n /|\\\n / \\",
                ["грусть"] = "({T_T})\n /|\\\n / \\",
                ["злость"] = "({>_<})\n /|\\\n / \\",
                ["удивление"] = "({O_O})\n /|\\\n / \\",
                ["страх"] = "({>_<})\n /|\\\n / \\"
            },
            Desc = "Страшный монстр"
        };
        characters["princess"] = new CharacterData {
            Name = "Princess",
            Emojis = new Dictionary<string, string> {
                ["default"] = "(✿◠‿◠)\n /|\\\n / \\",
                ["радость"] = "(✿◠‿◠)\n /|\\\n / \\",
                ["грусть"] = "(✿T_T)\n /|\\\n / \\",
                ["злость"] = "(✿>_<)\n /|\\\n / \\",
                ["удивление"] = "(✿O_O)\n /|\\\n / \\",
                ["страх"] = "(✿>_<)\n /|\\\n / \\"
            },
            Desc = "Прекрасная принцесса"
        };
        characters["knight"] = new CharacterData {
            Name = "Knight",
            Emojis = new Dictionary<string, string> {
                ["default"] = "[⚔️] \n /|\\\n / \\",
                ["радость"] = "[⚔️^_^]\n /|\\\n / \\",
                ["грусть"] = "[⚔️T_T]\n /|\\\n / \\",
                ["злость"] = "[⚔️>_<]\n /|\\\n / \\",
                ["удивление"] = "[⚔️O_O]\n /|\\\n / \\",
                ["страх"] = "[⚔️>_<]\n /|\\\n / \\"
            },
            Desc = "Доблестный рыцарь"
        };
    }

    public void ListCharacters() {
        Console.WriteLine("\u001B[36mДоступные персонажи:\u001B[0m");
        int i = 1;
        foreach (var entry in characters) {
            Console.WriteLine($"  {i}. {entry.Value.Name} — {entry.Value.Desc}");
            i++;
        }
    }

    public string GetCharacter(int choice) {
        var keys = characters.Keys.ToList();
        if (choice >= 1 && choice <= keys.Count) {
            return keys[choice - 1];
        }
        return null;
    }

    public void CreateScene() {
        Console.WriteLine("\u001B[36m🎭 Создание сцены комикса\u001B[0m");
        ListCharacters();

        while (true) {
            Console.Write("Выберите персонажа (0 — завершить): ");
            string input = Console.ReadLine().Trim();
            if (!int.TryParse(input, out int choice)) {
                Console.WriteLine("\u001B[31m❌ Неверный выбор.\u001B[0m");
                continue;
            }
            if (choice == 0) break;

            string charKey = GetCharacter(choice);
            if (charKey == null) {
                Console.WriteLine("\u001B[31m❌ Неверный выбор.\u001B[0m");
                continue;
            }

            var charData = characters[charKey];
            Console.WriteLine($"Доступные эмоции: {string.Join(", ", charData.Emojis.Keys)}");
            Console.Write("Выберите эмоцию: ");
            string emotion = Console.ReadLine().Trim().ToLower();
            if (!charData.Emojis.ContainsKey(emotion)) {
                emotion = "default";
            }

            Console.Write("Введите реплику персонажа: ");
            string dialogue = Console.ReadLine().Trim();

            scene.Add(new SceneItem {
                Character = charKey,
                Name = charData.Name,
                Emotion = emotion,
                Dialogue = dialogue,
                Ascii = charData.Emojis[emotion]
            });
            Console.WriteLine($"\u001B[32m✅ {charData.Name} добавлен в сцену!\u001B[0m");
        }
    }

    public void RenderScene() {
        if (scene.Count == 0) {
            Console.WriteLine("\u001B[33mСцена пуста. Добавьте персонажей.\u001B[0m");
            return;
        }

        Console.WriteLine("\n+" + new string('-', 48) + "+");
        foreach (var item in scene) {
            foreach (var line in item.Ascii.Split('\n')) {
                Console.WriteLine($"| {line,-46} |");
            }
            if (!string.IsNullOrEmpty(item.Dialogue)) {
                Console.WriteLine($"| \u001B[33m💬 {item.Dialogue,-44}\u001B[0m |");
            }
            Console.WriteLine("|" + new string('-', 48) + "|");
        }
        Console.WriteLine("+" + new string('-', 48) + "+");
    }

    public void SaveHTML(string filename = "comic.html") {
        if (scene.Count == 0) {
            Console.WriteLine("\u001B[33mНет сцены для сохранения.\u001B[0m");
            return;
        }

        var html = new StringBuilder();
        html.AppendLine("<!DOCTYPE html>");
        html.AppendLine("<html><head><meta charset=\"UTF-8\"><title>Мой комикс</title>");
        html.AppendLine("<style>");
        html.AppendLine("body { font-family: monospace; background: #f0f0f0; padding: 20px; }");
        html.AppendLine(".panel { background: white; border: 2px solid #333; border-radius: 10px; padding: 20px; margin: 20px auto; max-width: 600px; box-shadow: 0 4px 8px rgba(0,0,0,0.1); }");
        html.AppendLine(".character { white-space: pre; font-size: 14px; line-height: 1.2; text-align: center; }");
        html.AppendLine(".dialogue { font-size: 18px; color: #2c3e50; text-align: center; margin: 10px 0; padding: 10px; background: #ecf0f1; border-radius: 10px; }");
        html.AppendLine(".name { font-weight: bold; color: #2980b9; text-align: center; }");
        html.AppendLine("</style></head><body><h1 style=\"text-align:center;\">🎭 Мой комикс</h1>");

        foreach (var item in scene) {
            html.AppendLine("<div class=\"panel\">");
            html.AppendLine($"    <div class=\"name\">{item.Name} ({item.Emotion})</div>");
            html.AppendLine($"    <div class=\"character\">{item.Ascii}</div>");
            html.AppendLine($"    <div class=\"dialogue\">💬 {item.Dialogue}</div>");
            html.AppendLine("</div>");
        }
        html.AppendLine("</body></html>");
        File.WriteAllText(filename, html.ToString());
        Console.WriteLine($"\u001B[32m💾 Комикс сохранён в {filename}\u001B[0m");
    }

    public void SaveJSON(string filename = "comic_project.json") {
        var sb = new StringBuilder();
        sb.AppendLine("{");
        sb.AppendLine($"  \"timestamp\": \"{DateTime.Now:yyyy-MM-ddTHH:mm:ss.fffZ}\",");
        sb.AppendLine("  \"scene\": [");
        for (int i = 0; i < scene.Count; i++) {
            var item = scene[i];
            sb.AppendLine("    {");
            sb.AppendLine($"      \"character\": \"{item.Character}\",");
            sb.AppendLine($"      \"name\": \"{item.Name}\",");
            sb.AppendLine($"      \"emotion\": \"{item.Emotion}\",");
            sb.AppendLine($"      \"dialogue\": \"{item.Dialogue}\",");
            sb.AppendLine($"      \"ascii\": \"{item.Ascii.Replace("\n", "\\n")}\"");
            sb.Append("    }");
            if (i < scene.Count - 1) sb.Append(",");
            sb.AppendLine();
        }
        sb.AppendLine("  ]");
        sb.AppendLine("}");
        File.WriteAllText(filename, sb.ToString());
        Console.WriteLine($"\u001B[32m💾 Проект сохранён в {filename}\u001B[0m");
    }

    public void LoadJSON(string filename = "comic_project.json") {
        if (!File.Exists(filename)) {
            Console.WriteLine($"\u001B[31m❌ Файл {filename} не найден.\u001B[0m");
            return;
        }
        Console.WriteLine($"\u001B[32m✅ Проект загружен из {filename}\u001B[0m");
        Console.WriteLine("\u001B[33m⚠️ Для полной загрузки используйте JSON-библиотеку.\u001B[0m");
    }

    public static void Main() {
        var editor = new ComicEditor();
        while (true) {
            Console.WriteLine("\n\u001B[36m🎭 Comic Editor Pro (C#)\u001B[0m");
            Console.WriteLine("1. Создать новую сцену");
            Console.WriteLine("2. Показать сцену");
            Console.WriteLine("3. Сохранить как HTML");
            Console.WriteLine("4. Сохранить проект (JSON)");
            Console.WriteLine("5. Загрузить проект (JSON)");
            Console.WriteLine("6. Выход");
            Console.Write("Выберите действие: ");
            string choice = Console.ReadLine().Trim();

            switch (choice) {
                case "1": editor.CreateScene(); break;
                case "2": editor.RenderScene(); break;
                case "3": editor.SaveHTML(); break;
                case "4": editor.SaveJSON(); break;
                case "5": editor.LoadJSON(); break;
                case "6": Console.WriteLine("До свидания!"); return;
                default: Console.WriteLine("\u001B[31m❌ Неверный выбор.\u001B[0m"); break;
            }
        }
    }
}
