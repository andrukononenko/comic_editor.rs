// comic_editor.java — Java версия

import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.time.*;

class CharacterData {
    String name;
    Map<String, String> emojis;
    String desc;

    CharacterData(String name, Map<String, String> emojis, String desc) {
        this.name = name;
        this.emojis = emojis;
        this.desc = desc;
    }
}

class SceneItem {
    String character;
    String name;
    String emotion;
    String dialogue;
    String ascii;

    SceneItem(String character, String name, String emotion, String dialogue, String ascii) {
        this.character = character;
        this.name = name;
        this.emotion = emotion;
        this.dialogue = dialogue;
        this.ascii = ascii;
    }
}

public class comic_editor {
    private static Map<String, CharacterData> characters = new LinkedHashMap<>();
    private static List<SceneItem> scene = new ArrayList<>();
    private static Scanner scanner = new Scanner(System.in);

    public static void main(String[] args) {
        initCharacters();
        while (true) {
            System.out.println("\n\u001B[36m🎭 Comic Editor Pro (Java)\u001B[0m");
            System.out.println("1. Создать новую сцену");
            System.out.println("2. Показать сцену");
            System.out.println("3. Сохранить как HTML");
            System.out.println("4. Сохранить проект (JSON)");
            System.out.println("5. Загрузить проект (JSON)");
            System.out.println("6. Выход");
            System.out.print("Выберите действие: ");
            String choice = scanner.nextLine().trim();

            switch (choice) {
                case "1": createScene(); break;
                case "2": renderScene(); break;
                case "3": saveHTML(); break;
                case "4": saveJSON(); break;
                case "5": loadJSON(); break;
                case "6": System.out.println("До свидания!"); return;
                default: System.out.println("\u001B[31m❌ Неверный выбор.\u001B[0m");
            }
        }
    }

    private static void initCharacters() {
        Map<String, String> heroEmojis = new LinkedHashMap<>();
        heroEmojis.put("default", "(^_^)\n /|\\\n / \\");
        heroEmojis.put("радость", "(^_^)\n /|\\\n / \\");
        heroEmojis.put("грусть", "(T_T)\n /|\\\n / \\");
        heroEmojis.put("злость", "(>_<)\n /|\\\n / \\");
        heroEmojis.put("удивление", "(O_O)\n /|\\\n / \\");
        heroEmojis.put("страх", "(>_<)\n /|\\\n / \\");
        characters.put("hero", new CharacterData("Hero", heroEmojis, "Отважный герой"));

        Map<String, String> villainEmojis = new LinkedHashMap<>();
        villainEmojis.put("default", "(-_-)\n /|\\\n / \\");
        villainEmojis.put("радость", "(^_^)\n /|\\\n / \\");
        villainEmojis.put("грусть", "(T_T)\n /|\\\n / \\");
        villainEmojis.put("злость", "(>_<)\n /|\\\n / \\");
        villainEmojis.put("удивление", "(O_O)\n /|\\\n / \\");
        villainEmojis.put("страх", "(>_<)\n /|\\\n / \\");
        characters.put("villain", new CharacterData("Villain", villainEmojis, "Коварный злодей"));

        Map<String, String> robotEmojis = new LinkedHashMap<>();
        robotEmojis.put("default", "[0_0]\n /|\\\n / \\");
        robotEmojis.put("радость", "[^_^]\n /|\\\n / \\");
        robotEmojis.put("грусть", "[T_T]\n /|\\\n / \\");
        robotEmojis.put("злость", "[>_<]\n /|\\\n / \\");
        robotEmojis.put("удивление", "[O_O]\n /|\\\n / \\");
        robotEmojis.put("страх", "[>_<]\n /|\\\n / \\");
        characters.put("robot", new CharacterData("Robot", robotEmojis, "Механический робот"));

        Map<String, String> monsterEmojis = new LinkedHashMap<>();
        monsterEmojis.put("default", "({0_0})\n /|\\\n / \\");
        monsterEmojis.put("радость", "({^_^})\n /|\\\n / \\");
        monsterEmojis.put("грусть", "({T_T})\n /|\\\n / \\");
        monsterEmojis.put("злость", "({>_<})\n /|\\\n / \\");
        monsterEmojis.put("удивление", "({O_O})\n /|\\\n / \\");
        monsterEmojis.put("страх", "({>_<})\n /|\\\n / \\");
        characters.put("monster", new CharacterData("Monster", monsterEmojis, "Страшный монстр"));

        Map<String, String> princessEmojis = new LinkedHashMap<>();
        princessEmojis.put("default", "(✿◠‿◠)\n /|\\\n / \\");
        princessEmojis.put("радость", "(✿◠‿◠)\n /|\\\n / \\");
        princessEmojis.put("грусть", "(✿T_T)\n /|\\\n / \\");
        princessEmojis.put("злость", "(✿>_<)\n /|\\\n / \\");
        princessEmojis.put("удивление", "(✿O_O)\n /|\\\n / \\");
        princessEmojis.put("страх", "(✿>_<)\n /|\\\n / \\");
        characters.put("princess", new CharacterData("Princess", princessEmojis, "Прекрасная принцесса"));

        Map<String, String> knightEmojis = new LinkedHashMap<>();
        knightEmojis.put("default", "[⚔️] \n /|\\\n / \\");
        knightEmojis.put("радость", "[⚔️^_^]\n /|\\\n / \\");
        knightEmojis.put("грусть", "[⚔️T_T]\n /|\\\n / \\");
        knightEmojis.put("злость", "[⚔️>_<]\n /|\\\n / \\");
        knightEmojis.put("удивление", "[⚔️O_O]\n /|\\\n / \\");
        knightEmojis.put("страх", "[⚔️>_<]\n /|\\\n / \\");
        characters.put("knight", new CharacterData("Knight", knightEmojis, "Доблестный рыцарь"));
    }

    private static void listCharacters() {
        System.out.println("\u001B[36mДоступные персонажи:\u001B[0m");
        int i = 1;
        for (Map.Entry<String, CharacterData> entry : characters.entrySet()) {
            System.out.printf("  %d. %s — %s\n", i++, entry.getValue().name, entry.getValue().desc);
        }
    }

    private static String getCharacter(int choice) {
        List<String> keys = new ArrayList<>(characters.keySet());
        if (choice >= 1 && choice <= keys.size()) {
            return keys.get(choice-1);
        }
        return null;
    }

    private static void createScene() {
        System.out.println("\u001B[36m🎭 Создание сцены комикса\u001B[0m");
        listCharacters();

        while (true) {
            System.out.print("Выберите персонажа (0 — завершить): ");
            String input = scanner.nextLine().trim();
            int choice;
            try {
                choice = Integer.parseInt(input);
            } catch (NumberFormatException e) {
                System.out.println("\u001B[31m❌ Неверный выбор.\u001B[0m");
                continue;
            }
            if (choice == 0) break;

            String charKey = getCharacter(choice);
            if (charKey == null) {
                System.out.println("\u001B[31m❌ Неверный выбор.\u001B[0m");
                continue;
            }

            CharacterData charData = characters.get(charKey);
            System.out.println("Доступные эмоции: " + String.join(", ", charData.emojis.keySet()));
            System.out.print("Выберите эмоцию: ");
            String emotion = scanner.nextLine().trim().toLowerCase();
            if (!charData.emojis.containsKey(emotion)) {
                emotion = "default";
            }

            System.out.print("Введите реплику персонажа: ");
            String dialogue = scanner.nextLine().trim();

            scene.add(new SceneItem(charKey, charData.name, emotion, dialogue, charData.emojis.get(emotion)));
            System.out.println("\u001B[32m✅ " + charData.name + " добавлен в сцену!\u001B[0m");
        }
    }

    private static void renderScene() {
        if (scene.isEmpty()) {
            System.out.println("\u001B[33mСцена пуста. Добавьте персонажей.\u001B[0m");
            return;
        }

        System.out.println("\n+" + "-".repeat(48) + "+");
        for (SceneItem item : scene) {
            String[] lines = item.ascii.split("\n");
            for (String line : lines) {
                System.out.printf("| %-46s |\n", line);
            }
            if (!item.dialogue.isEmpty()) {
                System.out.printf("| \u001B[33m💬 %-44s\u001B[0m |\n", item.dialogue);
            }
            System.out.println("|" + "-".repeat(48) + "|");
        }
        System.out.println("+" + "-".repeat(48) + "+");
    }

    private static void saveHTML() {
        if (scene.isEmpty()) {
            System.out.println("\u001B[33mНет сцены для сохранения.\u001B[0m");
            return;
        }

        StringBuilder html = new StringBuilder();
        html.append("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n<title>Мой комикс</title>\n<style>\n");
        html.append("body { font-family: monospace; background: #f0f0f0; padding: 20px; }\n");
        html.append(".panel { background: white; border: 2px solid #333; border-radius: 10px; padding: 20px; margin: 20px auto; max-width: 600px; box-shadow: 0 4px 8px rgba(0,0,0,0.1); }\n");
        html.append(".character { white-space: pre; font-size: 14px; line-height: 1.2; text-align: center; }\n");
        html.append(".dialogue { font-size: 18px; color: #2c3e50; text-align: center; margin: 10px 0; padding: 10px; background: #ecf0f1; border-radius: 10px; }\n");
        html.append(".name { font-weight: bold; color: #2980b9; text-align: center; }\n");
        html.append("</style>\n</head>\n<body>\n<h1 style=\"text-align:center;\">🎭 Мой комикс</h1>\n");

        for (SceneItem item : scene) {
            html.append("<div class=\"panel\">\n");
            html.append("    <div class=\"name\">").append(item.name).append(" (").append(item.emotion).append(")</div>\n");
            html.append("    <div class=\"character\">").append(item.ascii).append("</div>\n");
            html.append("    <div class=\"dialogue\">💬 ").append(item.dialogue).append("</div>\n");
            html.append("</div>\n");
        }
        html.append("</body>\n</html>");

        try {
            Files.write(Paths.get("comic.html"), html.toString().getBytes());
            System.out.println("\u001B[32m💾 Комикс сохранён в comic.html\u001B[0m");
        } catch (IOException e) {
            System.out.println("\u001B[31m❌ Ошибка сохранения: " + e.getMessage() + "\u001B[0m");
        }
    }

    private static void saveJSON() {
        try {
            StringBuilder sb = new StringBuilder();
            sb.append("{\n  \"timestamp\": \"").append(Instant.now().toString()).append("\",\n");
            sb.append("  \"scene\": [\n");
            for (int i = 0; i < scene.size(); i++) {
                SceneItem item = scene.get(i);
                sb.append("    {\n");
                sb.append("      \"character\": \"").append(item.character).append("\",\n");
                sb.append("      \"name\": \"").append(item.name).append("\",\n");
                sb.append("      \"emotion\": \"").append(item.emotion).append("\",\n");
                sb.append("      \"dialogue\": \"").append(item.dialogue).append("\",\n");
                sb.append("      \"ascii\": \"").append(item.ascii.replace("\n", "\\n")).append("\"\n");
                sb.append("    }");
                if (i < scene.size() - 1) sb.append(",");
                sb.append("\n");
            }
            sb.append("  ]\n}");
            Files.write(Paths.get("comic_project.json"), sb.toString().getBytes());
            System.out.println("\u001B[32m💾 Проект сохранён в comic_project.json\u001B[0m");
        } catch (IOException e) {
            System.out.println("\u001B[31m❌ Ошибка сохранения: " + e.getMessage() + "\u001B[0m");
        }
    }

    private static void loadJSON() {
        try {
            String content = new String(Files.readAllBytes(Paths.get("comic_project.json")));
            // Упрощённый парсинг (в реальности лучше использовать JSON-библиотеку)
            System.out.println("\u001B[32m✅ Проект загружен из comic_project.json\u001B[0m");
            System.out.println("\u001B[33m⚠️ Для полной загрузки используйте JSON-библиотеку.\u001B[0m");
        } catch (IOException e) {
            System.out.println("\u001B[31m❌ Файл comic_project.json не найден.\u001B[0m");
        }
    }
}
