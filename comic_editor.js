// comic_editor.js — JavaScript версия

const fs = require('fs');
const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

class ComicEditor {
    constructor() {
        this.characters = this.initCharacters();
        this.scene = [];
    }

    initCharacters() {
        return {
            hero: {
                name: 'Hero',
                emojis: {
                    default: '(^_^)\n /|\\\n / \\',
                    радость: '(^_^)\n /|\\\n / \\',
                    грусть: '(T_T)\n /|\\\n / \\',
                    злость: '(>_<)\n /|\\\n / \\',
                    удивление: '(O_O)\n /|\\\n / \\',
                    страх: '(>_<)\n /|\\\n / \\'
                },
                desc: 'Отважный герой'
            },
            villain: {
                name: 'Villain',
                emojis: {
                    default: '(-_-)\n /|\\\n / \\',
                    радость: '(^_^)\n /|\\\n / \\',
                    грусть: '(T_T)\n /|\\\n / \\',
                    злость: '(>_<)\n /|\\\n / \\',
                    удивление: '(O_O)\n /|\\\n / \\',
                    страх: '(>_<)\n /|\\\n / \\'
                },
                desc: 'Коварный злодей'
            },
            robot: {
                name: 'Robot',
                emojis: {
                    default: '[0_0]\n /|\\\n / \\',
                    радость: '[^_^]\n /|\\\n / \\',
                    грусть: '[T_T]\n /|\\\n / \\',
                    злость: '[>_<]\n /|\\\n / \\',
                    удивление: '[O_O]\n /|\\\n / \\',
                    страх: '[>_<]\n /|\\\n / \\'
                },
                desc: 'Механический робот'
            },
            monster: {
                name: 'Monster',
                emojis: {
                    default: '({0_0})\n /|\\\n / \\',
                    радость: '({^_^})\n /|\\\n / \\',
                    грусть: '({T_T})\n /|\\\n / \\',
                    злость: '({>_<})\n /|\\\n / \\',
                    удивление: '({O_O})\n /|\\\n / \\',
                    страх: '({>_<})\n /|\\\n / \\'
                },
                desc: 'Страшный монстр'
            },
            princess: {
                name: 'Princess',
                emojis: {
                    default: '(✿◠‿◠)\n /|\\\n / \\',
                    радость: '(✿◠‿◠)\n /|\\\n / \\',
                    грусть: '(✿T_T)\n /|\\\n / \\',
                    злость: '(✿>_<)\n /|\\\n / \\',
                    удивление: '(✿O_O)\n /|\\\n / \\',
                    страх: '(✿>_<)\n /|\\\n / \\'
                },
                desc: 'Прекрасная принцесса'
            },
            knight: {
                name: 'Knight',
                emojis: {
                    default: '[⚔️] \n /|\\\n / \\',
                    радость: '[⚔️^_^]\n /|\\\n / \\',
                    грусть: '[⚔️T_T]\n /|\\\n / \\',
                    злость: '[⚔️>_<]\n /|\\\n / \\',
                    удивление: '[⚔️O_O]\n /|\\\n / \\',
                    страх: '[⚔️>_<]\n /|\\\n / \\'
                },
                desc: 'Доблестный рыцарь'
            }
        };
    }

    listCharacters() {
        console.log('\x1b[36mДоступные персонажи:\x1b[0m');
        const keys = Object.keys(this.characters);
        keys.forEach((key, i) => {
            const char = this.characters[key];
            console.log(`  ${i+1}. ${char.name} — ${char.desc}`);
        });
    }

    getCharacter(choice) {
        const keys = Object.keys(this.characters);
        if (choice >= 1 && choice <= keys.length) {
            return keys[choice-1];
        }
        return null;
    }

    async createScene() {
        console.log('\x1b[36m🎭 Создание сцены комикса\x1b[0m');
        this.listCharacters();

        while (true) {
            const choiceStr = await this.ask('Выберите персонажа (0 — завершить): ');
            const choice = parseInt(choiceStr);
            if (choice === 0) break;
            if (isNaN(choice)) {
                console.log('\x1b[31m❌ Неверный выбор.\x1b[0m');
                continue;
            }

            const charKey = this.getCharacter(choice);
            if (!charKey) {
                console.log('\x1b[31m❌ Неверный выбор.\x1b[0m');
                continue;
            }

            const char = this.characters[charKey];
            const emotions = Object.keys(char.emojis);
            console.log(`Доступные эмоции: ${emotions.join(', ')}`);
            const emotion = await this.ask('Выберите эмоцию: ');
            const finalEmotion = emotions.includes(emotion) ? emotion : 'default';

            const dialogue = await this.ask('Введите реплику персонажа: ');

            this.scene.push({
                character: charKey,
                name: char.name,
                emotion: finalEmotion,
                dialogue: dialogue,
                ascii: char.emojis[finalEmotion]
            });
            console.log(`\x1b[32m✅ ${char.name} добавлен в сцену!\x1b[0m`);
        }
    }

    renderScene() {
        if (this.scene.length === 0) {
            console.log('\x1b[33mСцена пуста. Добавьте персонажей.\x1b[0m');
            return;
        }

        console.log('\n+' + '-'.repeat(48) + '+');
        for (const item of this.scene) {
            const lines = item.ascii.split('\n');
            for (const line of lines) {
                console.log(`| ${line.padEnd(46)} |`);
            }
            if (item.dialogue) {
                console.log(`| \x1b[33m💬 ${item.dialogue.padEnd(44)}\x1b[0m |`);
            }
            console.log('|' + '-'.repeat(48) + '|');
        }
        console.log('+' + '-'.repeat(48) + '+');
    }

    saveHTML(filename = 'comic.html') {
        if (this.scene.length === 0) {
            console.log('\x1b[33mНет сцены для сохранения.\x1b[0m');
            return;
        }

        let html = `<!DOCTYPE html>
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
`;
        for (const item of this.scene) {
            html += `
<div class="panel">
    <div class="name">${item.name} (${item.emotion})</div>
    <div class="character">${item.ascii}</div>
    <div class="dialogue">💬 ${item.dialogue}</div>
</div>
`;
        }
        html += `
</body>
</html>`;
        fs.writeFileSync(filename, html);
        console.log(`\x1b[32m💾 Комикс сохранён в ${filename}\x1b[0m`);
    }

    saveJSON(filename = 'comic_project.json') {
        const data = {
            timestamp: new Date().toISOString(),
            scene: this.scene
        };
        fs.writeFileSync(filename, JSON.stringify(data, null, 2));
        console.log(`\x1b[32m💾 Проект сохранён в ${filename}\x1b[0m`);
    }

    loadJSON(filename = 'comic_project.json') {
        if (!fs.existsSync(filename)) {
            console.log(`\x1b[31m❌ Файл ${filename} не найден.\x1b[0m`);
            return;
        }
        const data = JSON.parse(fs.readFileSync(filename, 'utf8'));
        this.scene = data.scene || [];
        console.log(`\x1b[32m✅ Проект загружен из ${filename}\x1b[0m`);
    }

    ask(question) {
        return new Promise(resolve => rl.question(question, resolve));
    }
}

async function main() {
    const editor = new ComicEditor();
    while (true) {
        console.log('\n\x1b[36m🎭 Comic Editor Pro (JavaScript)\x1b[0m');
        console.log('1. Создать новую сцену');
        console.log('2. Показать сцену');
        console.log('3. Сохранить как HTML');
        console.log('4. Сохранить проект (JSON)');
        console.log('5. Загрузить проект (JSON)');
        console.log('6. Выход');
        const choice = await editor.ask('Выберите действие: ');

        switch (choice.trim()) {
            case '1': await editor.createScene(); break;
            case '2': editor.renderScene(); break;
            case '3': editor.saveHTML(); break;
            case '4': editor.saveJSON(); break;
            case '5': editor.loadJSON(); break;
            case '6': console.log('До свидания!'); rl.close(); return;
            default: console.log('\x1b[31m❌ Неверный выбор.\x1b[0m');
        }
    }
}

main().catch(console.error);
