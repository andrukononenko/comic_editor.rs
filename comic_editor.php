<?php
// comic_editor.php — PHP версия

class ComicEditor {
    private $characters = [];
    private $scene = [];

    public function __construct() {
        $this->initCharacters();
    }

    private function initCharacters() {
        $this->characters = [
            'hero' => [
                'name' => 'Hero',
                'emojis' => [
                    'default' => "(^_^)\n /|\\\n / \\",
                    'радость' => "(^_^)\n /|\\\n / \\",
                    'грусть' => "(T_T)\n /|\\\n / \\",
                    'злость' => "(>_<)\n /|\\\n / \\",
                    'удивление' => "(O_O)\n /|\\\n / \\",
                    'страх' => "(>_<)\n /|\\\n / \\"
                ],
                'desc' => 'Отважный герой'
            ],
            'villain' => [
                'name' => 'Villain',
                'emojis' => [
                    'default' => "(-_-)\n /|\\\n / \\",
                    'радость' => "(^_^)\n /|\\\n / \\",
                    'грусть' => "(T_T)\n /|\\\n / \\",
                    'злость' => "(>_<)\n /|\\\n / \\",
                    'удивление' => "(O_O)\n /|\\\n / \\",
                    'страх' => "(>_<)\n /|\\\n / \\"
                ],
                'desc' => 'Коварный злодей'
            ],
            'robot' => [
                'name' => 'Robot',
                'emojis' => [
                    'default' => "[0_0]\n /|\\\n / \\",
                    'радость' => "[^_^]\n /|\\\n / \\",
                    'грусть' => "[T_T]\n /|\\\n / \\",
                    'злость' => "[>_<]\n /|\\\n / \\",
                    'удивление' => "[O_O]\n /|\\\n / \\",
                    'страх' => "[>_<]\n /|\\\n / \\"
                ],
                'desc' => 'Механический робот'
            ],
            'monster' => [
                'name' => 'Monster',
                'emojis' => [
                    'default' => "({0_0})\n /|\\\n / \\",
                    'радость' => "({^_^})\n /|\\\n / \\",
                    'грусть' => "({T_T})\n /|\\\n / \\",
                    'злость' => "({>_<})\n /|\\\n / \\",
                    'удивление' => "({O_O})\n /|\\\n / \\",
                    'страх' => "({>_<})\n /|\\\n / \\"
                ],
                'desc' => 'Страшный монстр'
            ],
            'princess' => [
                'name' => 'Princess',
                'emojis' => [
                    'default' => "(✿◠‿◠)\n /|\\\n / \\",
                    'радость' => "(✿◠‿◠)\n /|\\\n / \\",
                    'грусть' => "(✿T_T)\n /|\\\n / \\",
                    'злость' => "(✿>_<)\n /|\\\n / \\",
                    'удивление' => "(✿O_O)\n /|\\\n / \\",
                    'страх' => "(✿>_<)\n /|\\\n / \\"
                ],
                'desc' => 'Прекрасная принцесса'
            ],
            'knight' => [
                'name' => 'Knight',
                'emojis' => [
                    'default' => "[⚔️] \n /|\\\n / \\",
                    'радость' => "[⚔️^_^]\n /|\\\n / \\",
                    'грусть' => "[⚔️T_T]\n /|\\\n / \\",
                    'злость' => "[⚔️>_<]\n /|\\\n / \\",
                    'удивление' => "[⚔️O_O]\n /|\\\n / \\",
                    'страх' => "[⚔️>_<]\n /|\\\n / \\"
                ],
                'desc' => 'Доблестный рыцарь'
            ]
        ];
    }

    private function listCharacters() {
        echo "\033[36mДоступные персонажи:\033[0m\n";
        $i = 1;
        foreach ($this->characters as $key => $char) {
            echo "  $i. {$char['name']} — {$char['desc']}\n";
            $i++;
        }
    }

    private function getCharacter($choice) {
        $keys = array_keys($this->characters);
        if ($choice >= 1 && $choice <= count($keys)) {
            return $keys[$choice-1];
        }
        return null;
    }

    public function createScene() {
        echo "\033[36m🎭 Создание сцены комикса\033[0m\n";
        $this->listCharacters();

        while (true) {
            echo "Выберите персонажа (0 — завершить): ";
            $input = trim(fgets(STDIN));
            $choice = (int)$input;
            if ($choice == 0) break;

            $charKey = $this->getCharacter($choice);
            if (!$charKey) {
                echo "\033[31m❌ Неверный выбор.\033[0m\n";
                continue;
            }

            $char = $this->characters[$charKey];
            echo "Доступные эмоции: " . implode(', ', array_keys($char['emojis'])) . "\n";
            echo "Выберите эмоцию: ";
            $emotion = trim(fgets(STDIN));
            $emotion = strtolower($emotion);
            if (!isset($char['emojis'][$emotion])) {
                $emotion = 'default';
            }

            echo "Введите реплику персонажа: ";
            $dialogue = trim(fgets(STDIN));

            $this->scene[] = [
                'character' => $charKey,
                'name' => $char['name'],
                'emotion' => $emotion,
                'dialogue' => $dialogue,
                'ascii' => $char['emojis'][$emotion]
            ];
            echo "\033[32m✅ {$char['name']} добавлен в сцену!\033[0m\n";
        }
    }

    public function renderScene() {
        if (empty($this->scene)) {
            echo "\033[33mСцена пуста. Добавьте персонажей.\033[0m\n";
            return;
        }

        echo "\n+" . str_repeat('-', 48) . "+\n";
        foreach ($this->scene as $item) {
            $lines = explode("\n", $item['ascii']);
            foreach ($lines as $line) {
                echo "| " . str_pad($line, 46) . " |\n";
            }
            if (!empty($item['dialogue'])) {
                echo "| \033[33m💬 " . str_pad($item['dialogue'], 44) . "\033[0m |\n";
            }
            echo "|" . str_repeat('-', 48) . "|\n";
        }
        echo "+" . str_repeat('-', 48) . "+\n";
    }

    public function saveHTML($filename = 'comic.html') {
        if (empty($this->scene)) {
            echo "\033[33mНет сцены для сохранения.\033[0m\n";
            return;
        }

        $html = '<!DOCTYPE html>
<html>
<head><meta charset="UTF-8"><title>Мой комикс</title>
<style>
body { font-family: monospace; background: #f0f0f0; padding: 20px; }
.panel { background: white; border: 2px solid #333; border-radius: 10px; padding: 20px; margin: 20px auto; max-width: 600px; box-shadow: 0 4px 8px rgba(0,0,0,0.1); }
.character { white-space: pre; font-size: 14px; line-height: 1.2; text-align: center; }
.dialogue { font-size: 18px; color: #2c3e50; text-align: center; margin: 10px 0; padding: 10px; background: #ecf0f1; border-radius: 10px; }
.name { font-weight: bold; color: #2980b9; text-align: center; }
</style>
</head>
<body>
<h1 style="text-align:center;">🎭 Мой комикс</h1>';

        foreach ($this->scene as $item) {
            $html .= "
<div class=\"panel\">
    <div class=\"name\">{$item['name']} ({$item['emotion']})</div>
    <div class=\"character\">{$item['ascii']}</div>
    <div class=\"dialogue\">💬 {$item['dialogue']}</div>
</div>";
        }

        $html .= '</body></html>';
        file_put_contents($filename, $html);
        echo "\033[32m💾 Комикс сохранён в $filename\033[0m\n";
    }

    public function saveJSON($filename = 'comic_project.json') {
        $data = [
            'timestamp' => date('c'),
            'scene' => $this->scene
        ];
        file_put_contents($filename, json_encode($data, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE));
        echo "\033[32m💾 Проект сохранён в $filename\033[0m\n";
    }

    public function loadJSON($filename = 'comic_project.json') {
        if (!file_exists($filename)) {
            echo "\033[31m❌ Файл $filename не найден.\033[0m\n";
            return;
        }
        $data = json_decode(file_get_contents($filename), true);
        $this->scene = $data['scene'] ?? [];
        echo "\033[32m✅ Проект загружен из $filename\033[0m\n";
    }
}

function main() {
    $editor = new ComicEditor();
    while (true) {
        echo "\n\033[36m🎭 Comic Editor Pro (PHP)\033[0m\n";
        echo "1. Создать новую сцену\n";
        echo "2. Показать сцену\n";
        echo "3. Сохранить как HTML\n";
        echo "4. Сохранить проект (JSON)\n";
        echo "5. Загрузить проект (JSON)\n";
        echo "6. Выход\n";
        echo "Выберите действие: ";
        $choice = trim(fgets(STDIN));

        switch ($choice) {
            case '1': $editor->createScene(); break;
            case '2': $editor->renderScene(); break;
            case '3': $editor->saveHTML(); break;
            case '4': $editor->saveJSON(); break;
            case '5': $editor->loadJSON(); break;
            case '6':
                echo "До свидания!\n";
                exit(0);
            default:
                echo "\033[31m❌ Неверный выбор.\033[0m\n";
        }
    }
}

main();
?>
