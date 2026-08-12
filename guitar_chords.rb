# guitar_chords.rb — Ruby версия

class GuitarChord
  def initialize
    @chords = {
      'C' => {
        'major' => {5=>3, 4=>2, 3=>0, 2=>1, 1=>0},
        'minor' => {5=>3, 4=>5, 3=>5, 2=>4, 1=>3},
        'seventh' => {5=>3, 4=>2, 3=>3, 2=>1, 1=>0}
      },
      'D' => {
        'major' => {4=>0, 3=>2, 2=>3, 1=>2},
        'minor' => {4=>0, 3=>2, 2=>3, 1=>1},
        'seventh' => {4=>0, 3=>2, 2=>1, 1=>2}
      },
      'E' => {
        'major' => {5=>0, 4=>2, 3=>2, 2=>1, 1=>0, 0=>0},
        'minor' => {5=>0, 4=>2, 3=>2, 2=>0, 1=>0, 0=>0},
        'seventh' => {5=>0, 4=>2, 3=>0, 2=>1, 1=>0, 0=>0}
      },
      'F' => {
        'major' => {5=>1, 4=>3, 3=>3, 2=>2, 1=>1, 0=>1},
        'minor' => {5=>1, 4=>3, 3=>3, 2=>1, 1=>1, 0=>1},
        'seventh' => {5=>1, 4=>3, 3=>1, 2=>2, 1=>1, 0=>1}
      },
      'G' => {
        'major' => {5=>3, 4=>2, 3=>0, 2=>0, 1=>0, 0=>3},
        'minor' => {5=>3, 4=>5, 3=>5, 2=>3, 1=>3, 0=>3},
        'seventh' => {5=>3, 4=>2, 3=>0, 2=>0, 1=>0, 0=>1}
      },
      'A' => {
        'major' => {5=>0, 4=>2, 3=>2, 2=>2, 1=>0},
        'minor' => {5=>0, 4=>2, 3=>2, 2=>1, 1=>0},
        'seventh' => {5=>0, 4=>2, 3=>0, 2=>2, 1=>0}
      },
      'B' => {
        'major' => {5=>2, 4=>4, 3=>4, 2=>4, 1=>2},
        'minor' => {5=>2, 4=>4, 3=>4, 2=>3, 1=>2},
        'seventh' => {5=>2, 4=>1, 3=>2, 2=>0, 1=>2}
      }
    }
  end

  def get_string(pos, str)
    if pos.key?(str)
      pos[str] == 0 ? 'o' : pos[str].to_s
    else
      'x'
    end
  end

  def print_fretboard(pos)
    labels = ['E', 'A', 'D', 'G', 'B', 'e']
    puts "\n   e B G D A E"
    puts "   ────────────"
    (0..5).each do |fret|
      line = fret == 0 ? '  ' : "#{fret} "
      line << ' |'
      (0..5).each do |str|
        if pos.key?(str) && pos[str] == fret
          line << ' ● |'
        elsif pos.key?(str) && pos[str] < fret
          line << '   |'
        else
          line << '   |'
        end
      end
      puts line
    end
    puts "\nАппликатура:"
    (0..5).each do |str|
      if pos.key?(str)
        if pos[str] == 0
          puts "  #{labels[str]}: открытая"
        else
          puts "  #{labels[str]}: #{pos[str]}-й лад"
        end
      end
    end
  end

  def get_chord(root, type)
    @chords.dig(root, type)
  end

  def list_chords
    puts "Доступные аккорды:"
    @chords.keys.sort.each do |root|
      types = @chords[root].keys.join(', ')
      puts "  #{root}: #{types}"
    end
  end
end

def main
  chord_gen = GuitarChord.new
  puts "🎸 Guitar Chord Generator (Ruby)"
  puts "Введите аккорд в формате: <корень> <тип>"
  puts "Пример: C major, D minor, E seventh"
  puts "Или введите 'list' для просмотра всех аккордов"
  puts "Введите 'exit' для выхода"

  loop do
    print "\n> "
    input = gets.chomp.strip
    break if input == 'exit' || input == 'quit'

    if input == 'list'
      chord_gen.list_chords
      next
    end

    parts = input.split
    if parts.size < 2
      puts "Неверный формат. Используйте: <корень> <тип>"
      next
    end

    root = parts[0].upcase
    type = parts[1].downcase

    pos = chord_gen.get_chord(root, type)
    if pos.nil?
      puts "❌ Аккорд #{root} #{type} не найден."
      puts "Используйте 'list' для просмотра всех доступных аккордов."
      next
    end

    puts "\n🎸 Аккорд: #{root} (#{type})"
    chord_gen.print_fretboard(pos)
  end

  puts "До свидания!"
end

main if __FILE__ == $0
