# comic_editor.rb — Ruby версия

require 'json'
require 'time'

class ComicEditor
  def initialize
    @characters = init_characters
    @scene = []
  end

  def init_characters
    {
      'hero' => {
        name: 'Hero',
        emojis: {
          'default' => "(^_^)\n /|\\\n / \\",
          'радость' => "(^_^)\n /|\\\n / \\",
          'грусть' => "(T_T)\n /|\\\n / \\",
          'злость' => "(>_<)\n /|\\\n / \\",
          'удивление' => "(O_O)\n /|\\\n / \\",
          'страх' => "(>_<)\n /|\\\n / \\"
        },
        desc: 'Отважный герой'
      },
      'villain' => {
        name: 'Villain',
        emojis: {
          'default' => "(-_-)\n /|\\\n / \\",
          'радость' => "(^_^)\n /|\\\n / \\",
          'грусть' => "(T_T)\n /|\\\n / \\",
          'злость' => "(>_<)\n /|\\\n / \\",
          'удивление' => "(O_O)\n /|\\\n / \\",
          'страх' => "(>_<)\n /|\\\n / \\"
        },
        desc: 'Коварный злодей'
      },
      'robot' => {
        name: 'Robot',
        emojis: {
          'default' => "[0_0]\n /|\\\n / \\",
          'радость' => "[^_^]\n /|\\\n / \\",
          'грусть' => "[T_T]\n /|\\\n / \\",
          'злость' => "[>_<]\n /|\\\n / \\",
          'удивление' => "[O_O]\n /|\\\n / \\",
          'страх' => "[>_<]\n /|\\\n / \\"
        },
        desc: 'Механический робот'
      },
      'monster' => {
        name: 'Monster',
        emojis: {
          'default' => "({0_0})\n /|\\\n / \\",
          'радость' => "({^_^})\n /|\\\n / \\",
          'грусть' => "({T_T})\n /|\\\n / \\",
          'злость' => "({>_<})\n /|\\\n / \\",
          'удивление' => "({O_O})\n /|\\\n / \\",
          'страх' => "({>_<})\n /|\\\n / \\"
        },
        desc: 'Страшный монстр'
      },
      'princess' => {
        name: 'Princess',
        emojis: {
          'default' => "(✿◠‿◠)\n /|\\\n / \\",
          'радость' => "(✿◠‿◠)\n /|\\\n / \\",
          'грусть' => "(✿T_T)\n /|\\\n / \\",
          'злость' => "(✿>_<)\n /|\\\n / \\",
          'удивление' => "(✿O_O)\n /|\\\n / \\",
          'страх' => "(✿>_<)\n /|\\\n / \\"
        },
        desc: 'Прекрасная принцесса'
      },
      'knight' => {
        name: 'Knight',
        emojis: {
          'default' => "[⚔️] \n /|\\\n / \\",
          'радость' => "[⚔️^_^]\n /|\\\n / \\",
          'грусть' => "[⚔️T_T]\n /|\\\n / \\",
          'злость' => "[⚔️>_<]\n /|\\\n / \\",
          'удивление' => "[⚔️O_O]\n /|\\\n / \\",
          'страх' => "[⚔️>_<]\n /|\\\n / \\"
        },
        desc: 'Доблестный рыцарь'
      }
    }
  end

  def list_characters
    puts "\e[36mДоступные персонажи:\e[0m"
    @characters.each_with_index do |(key, char), i|
      puts "  #{i+1}. #{char[:name]} — #{char[:desc]}"
    end
  end

  def get_character(choice)
    keys = @characters.keys
    if choice >= 1 && choice <= keys.size
      keys[choice-1]
    else
      nil
    end
  end

  def create_scene
    puts "\e[36m🎭 Создание сцены комикса\e[0m"
    list_characters

    loop do
      print "Выберите персонажа (0 — завершить): "
      input = gets.chomp
      choice = input.to_i
      break if choice == 0

      char_key = get_character(choice)
      if char_key.nil?
        puts "\e[31m❌ Неверный выбор.\e[0m"
        next
      end

      char = @characters[char_key]
      puts "Доступные эмоции: #{char[:emojis].keys.join(', ')}"
      print "Выберите эмоцию: "
      emotion = gets.chomp.downcase
      emotion = 'default' unless char[:emojis].key?(emotion)

      print "Введите реплику персонажа: "
      dialogue = gets.chomp

      @scene << {
        character: char_key,
        name: char[:name],
        emotion: emotion,
        dialogue: dialogue,
        ascii: char[:emojis][emotion]
      }
      puts "\e[32m✅ #{char[:name]} добавлен в сцену!\e[0m"
    end
  end

  def render_scene
    if @scene.empty?
      puts "\e[33mСцена пуста. Добавьте персонажей.\e[0m"
      return
    end

    puts "\n+" + "-" * 48 + "+"
    @scene.each do |item|
      item[:ascii].lines.each do |line|
        puts "| #{line.strip.ljust(46)} |"
      end
      unless item[:dialogue].empty?
        puts "| \e[33m💬 #{item[:dialogue].ljust(44)}\e[0m |"
      end
      puts "|" + "-" * 48 + "|"
    end
    puts "+" + "-" * 48 + "+"
  end

  def save_html(filename = 'comic.html')
    if @scene.empty?
      puts "\e[33mНет сцены для сохранения.\e[0m"
      return
    end

    html = <<~HTML
      <!DOCTYPE html>
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
      <h1 style="text-align:center;">🎭 Мой комикс</h1>
    HTML

    @scene.each do |item|
      html += <<~HTML
        <div class="panel">
            <div class="name">#{item[:name]} (#{item[:emotion]})</div>
            <div class="character">#{item[:ascii]}</div>
            <div class="dialogue">💬 #{item[:dialogue]}</div>
        </div>
      HTML
    end

    html += "</body></html>"
    File.write(filename, html)
    puts "\e[32m💾 Комикс сохранён в #{filename}\e[0m"
  end

  def save_json(filename = 'comic_project.json')
    data = {
      timestamp: Time.now.iso8601,
      scene: @scene
    }
    File.write(filename, JSON.pretty_generate(data))
    puts "\e[32m💾 Проект сохранён в #{filename}\e[0m"
  end

  def load_json(filename = 'comic_project.json')
    unless File.exist?(filename)
      puts "\e[31m❌ Файл #{filename} не найден.\e[0m"
      return
    end
    data = JSON.parse(File.read(filename), symbolize_names: true)
    @scene = data[:scene] || []
    puts "\e[32m✅ Проект загружен из #{filename}\e[0m"
  end
end

def main
  editor = ComicEditor.new
  loop do
    puts "\n\e[36m🎭 Comic Editor Pro (Ruby)\e[0m"
    puts "1. Создать новую сцену"
    puts "2. Показать сцену"
    puts "3. Сохранить как HTML"
    puts "4. Сохранить проект (JSON)"
    puts "5. Загрузить проект (JSON)"
    puts "6. Выход"
    print "Выберите действие: "
    choice = gets.chomp

    case choice
    when "1" then editor.create_scene
    when "2" then editor.render_scene
    when "3" then editor.save_html
    when "4" then editor.save_json
    when "5" then editor.load_json
    when "6"
      puts "До свидания!"
      break
    else
      puts "\e[31m❌ Неверный выбор.\e[0m"
    end
  end
end

main if __FILE__ == $0
