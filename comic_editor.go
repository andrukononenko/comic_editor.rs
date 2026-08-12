// comic_editor.go — Go версия

package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

type Character struct {
	Name    string            `json:"name"`
	Emojis  map[string]string `json:"emojis"`
	Desc    string            `json:"desc"`
}

type SceneItem struct {
	Character string `json:"character"`
	Name      string `json:"name"`
	Emotion   string `json:"emotion"`
	Dialogue  string `json:"dialogue"`
	Ascii     string `json:"ascii"`
}

type Project struct {
	Timestamp string      `json:"timestamp"`
	Scene     []SceneItem `json:"scene"`
}

type ComicEditor struct {
	characters map[string]Character
	scene      []SceneItem
	reader     *bufio.Reader
}

func NewComicEditor() *ComicEditor {
	return &ComicEditor{
		characters: initCharacters(),
		scene:      []SceneItem{},
		reader:     bufio.NewReader(os.Stdin),
	}
}

func initCharacters() map[string]Character {
	return map[string]Character{
		"hero": {
			Name: "Hero",
			Emojis: map[string]string{
				"default": "(^_^)\n /|\\\n / \\",
				"радость": "(^_^)\n /|\\\n / \\",
				"грусть":  "(T_T)\n /|\\\n / \\",
				"злость":  "(>_<)\n /|\\\n / \\",
				"удивление": "(O_O)\n /|\\\n / \\",
				"страх":   "(>_<)\n /|\\\n / \\",
			},
			Desc: "Отважный герой",
		},
		"villain": {
			Name: "Villain",
			Emojis: map[string]string{
				"default": "(-_-)\n /|\\\n / \\",
				"радость": "(^_^)\n /|\\\n / \\",
				"грусть":  "(T_T)\n /|\\\n / \\",
				"злость":  "(>_<)\n /|\\\n / \\",
				"удивление": "(O_O)\n /|\\\n / \\",
				"страх":   "(>_<)\n /|\\\n / \\",
			},
			Desc: "Коварный злодей",
		},
		"robot": {
			Name: "Robot",
			Emojis: map[string]string{
				"default": "[0_0]\n /|\\\n / \\",
				"радость": "[^_^]\n /|\\\n / \\",
				"грусть":  "[T_T]\n /|\\\n / \\",
				"злость":  "[>_<]\n /|\\\n / \\",
				"удивление": "[O_O]\n /|\\\n / \\",
				"страх":   "[>_<]\n /|\\\n / \\",
			},
			Desc: "Механический робот",
		},
		"monster": {
			Name: "Monster",
			Emojis: map[string]string{
				"default": "({0_0})\n /|\\\n / \\",
				"радость": "({^_^})\n /|\\\n / \\",
				"грусть":  "({T_T})\n /|\\\n / \\",
				"злость":  "({>_<})\n /|\\\n / \\",
				"удивление": "({O_O})\n /|\\\n / \\",
				"страх":   "({>_<})\n /|\\\n / \\",
			},
			Desc: "Страшный монстр",
		},
		"princess": {
			Name: "Princess",
			Emojis: map[string]string{
				"default": "(✿◠‿◠)\n /|\\\n / \\",
				"радость": "(✿◠‿◠)\n /|\\\n / \\",
				"грусть":  "(✿T_T)\n /|\\\n / \\",
				"злость":  "(✿>_<)\n /|\\\n / \\",
				"удивление": "(✿O_O)\n /|\\\n / \\",
				"страх":   "(✿>_<)\n /|\\\n / \\",
			},
			Desc: "Прекрасная принцесса",
		},
		"knight": {
			Name: "Knight",
			Emojis: map[string]string{
				"default": "[⚔️] \n /|\\\n / \\",
				"радость": "[⚔️^_^]\n /|\\\n / \\",
				"грусть":  "[⚔️T_T]\n /|\\\n / \\",
				"злость":  "[⚔️>_<]\n /|\\\n / \\",
				"удивление": "[⚔️O_O]\n /|\\\n / \\",
				"страх":   "[⚔️>_<]\n /|\\\n / \\",
			},
			Desc: "Доблестный рыцарь",
		},
	}
}

func (e *ComicEditor) listCharacters() {
	fmt.Println("\033[36mДоступные персонажи:\033[0m")
	i := 1
	for key, char := range e.characters {
		fmt.Printf("  %d. %s — %s\n", i, char.Name, char.Desc)
		i++
	}
}

func (e *ComicEditor) getCharacter(choice int) string {
	keys := []string{}
	for k := range e.characters {
		keys = append(keys, k)
	}
	if choice >= 1 && choice <= len(keys) {
		return keys[choice-1]
	}
	return ""
}

func (e *ComicEditor) createScene() {
	fmt.Println("\033[36m🎭 Создание сцены комикса\033[0m")
	e.listCharacters()

	for {
		fmt.Print("Выберите персонажа (0 — завершить): ")
		input, _ := e.reader.ReadString('\n')
		input = strings.TrimSpace(input)
		choice, err := strconv.Atoi(input)
		if err != nil {
			fmt.Println("\033[31m❌ Неверный выбор.\033[0m")
			continue
		}
		if choice == 0 {
			break
		}
		charKey := e.getCharacter(choice)
		if charKey == "" {
			fmt.Println("\033[31m❌ Неверный выбор.\033[0m")
			continue
		}

		char := e.characters[charKey]
		emotions := []string{}
		for e := range char.Emojis {
			emotions = append(emotions, e)
		}
		fmt.Printf("Доступные эмоции: %s\n", strings.Join(emotions, ", "))
		fmt.Print("Выберите эмоцию: ")
		emotion, _ := e.reader.ReadString('\n')
		emotion = strings.TrimSpace(strings.ToLower(emotion))
		if _, ok := char.Emojis[emotion]; !ok {
			emotion = "default"
		}

		fmt.Print("Введите реплику персонажа: ")
		dialogue, _ := e.reader.ReadString('\n')
		dialogue = strings.TrimSpace(dialogue)

		e.scene = append(e.scene, SceneItem{
			Character: charKey,
			Name:      char.Name,
			Emotion:   emotion,
			Dialogue:  dialogue,
			Ascii:     char.Emojis[emotion],
		})
		fmt.Printf("\033[32m✅ %s добавлен в сцену!\033[0m\n", char.Name)
	}
}

func (e *ComicEditor) renderScene() {
	if len(e.scene) == 0 {
		fmt.Println("\033[33mСцена пуста. Добавьте персонажей.\033[0m")
		return
	}

	fmt.Println("\n+" + strings.Repeat("-", 48) + "+")
	for _, item := range e.scene {
		lines := strings.Split(item.Ascii, "\n")
		for _, line := range lines {
			fmt.Printf("| %-46s |\n", line)
		}
		if item.Dialogue != "" {
			fmt.Printf("| \033[33m💬 %-44s\033[0m |\n", item.Dialogue)
		}
		fmt.Println("|" + strings.Repeat("-", 48) + "|")
	}
	fmt.Println("+" + strings.Repeat("-", 48) + "+")
}

func (e *ComicEditor) saveHTML(filename string) {
	if len(e.scene) == 0 {
		fmt.Println("\033[33mНет сцены для сохранения.\033[0m")
		return
	}

	html := `<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>Мой комикс</title>
<style>
body { font-family: monospace; background: #f0f0f0; padding: 20px; }
.panel { background: white; border: 2px solid #333; border-radius: 10px; padding: 20px; margin: 20px auto; max-width: 600px; box-shadow: 0 4px 8px rgba(0,0,0,0.1); }
.character { white-space: pre; font-size: 14px; line-height: 1.2; text-align: center; }
.dialogue { font-size: 18px; color: #2c3e50; text-align: center; margin: 10px 0; padding: 10px; background: #ecf0f1; border-radius: 10px; }
.name { font-weight: bold; color: #2980b9; text-align: center; }
</style>
</head>
<body>
<h1 style="text-align:center;">🎭 Мой комикс</h1>
`
	for _, item := range e.scene {
		html += fmt.Sprintf(`
<div class="panel">
    <div class="name">%s (%s)</div>
    <div class="character">%s</div>
    <div class="dialogue">💬 %s</div>
</div>
`, item.Name, item.Emotion, item.Ascii, item.Dialogue)
	}
	html += `
</body>
</html>
`
	os.WriteFile(filename, []byte(html), 0644)
	fmt.Printf("\033[32m💾 Комикс сохранён в %s\033[0m\n", filename)
}

func (e *ComicEditor) saveJSON(filename string) {
	data := Project{
		Timestamp: time.Now().Format(time.RFC3339),
		Scene:     e.scene,
	}
	jsonData, _ := json.MarshalIndent(data, "", "  ")
	os.WriteFile(filename, jsonData, 0644)
	fmt.Printf("\033[32m💾 Проект сохранён в %s\033[0m\n", filename)
}

func (e *ComicEditor) loadJSON(filename string) {
	data, err := os.ReadFile(filename)
	if err != nil {
		fmt.Printf("\033[31m❌ Файл %s не найден.\033[0m\n", filename)
		return
	}
	var project Project
	json.Unmarshal(data, &project)
	e.scene = project.Scene
	fmt.Printf("\033[32m✅ Проект загружен из %s\033[0m\n", filename)
}

func main() {
	editor := NewComicEditor()
	reader := bufio.NewReader(os.Stdin)

	for {
		fmt.Println("\n\033[36m🎭 Comic Editor Pro (Go)\033[0m")
		fmt.Println("1. Создать новую сцену")
		fmt.Println("2. Показать сцену")
		fmt.Println("3. Сохранить как HTML")
		fmt.Println("4. Сохранить проект (JSON)")
		fmt.Println("5. Загрузить проект (JSON)")
		fmt.Println("6. Выход")
		fmt.Print("Выберите действие: ")
		choice, _ := reader.ReadString('\n')
		choice = strings.TrimSpace(choice)

		switch choice {
		case "1":
			editor.createScene()
		case "2":
			editor.renderScene()
		case "3":
			editor.saveHTML("comic.html")
		case "4":
			editor.saveJSON("comic_project.json")
		case "5":
			editor.loadJSON("comic_project.json")
		case "6":
			fmt.Println("До свидания!")
			return
		default:
			fmt.Println("\033[31m❌ Неверный выбор.\033[0m")
		}
	}
}
