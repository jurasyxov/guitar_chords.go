<?php
// guitar_chords.php — PHP версия

class GuitarChord {
    private $chords;

    public function __construct() {
        $this->chords = [
            'C' => [
                'major' => [5=>3, 4=>2, 3=>0, 2=>1, 1=>0],
                'minor' => [5=>3, 4=>5, 3=>5, 2=>4, 1=>3],
                'seventh' => [5=>3, 4=>2, 3=>3, 2=>1, 1=>0]
            ],
            'D' => [
                'major' => [4=>0, 3=>2, 2=>3, 1=>2],
                'minor' => [4=>0, 3=>2, 2=>3, 1=>1],
                'seventh' => [4=>0, 3=>2, 2=>1, 1=>2]
            ],
            'E' => [
                'major' => [5=>0, 4=>2, 3=>2, 2=>1, 1=>0, 0=>0],
                'minor' => [5=>0, 4=>2, 3=>2, 2=>0, 1=>0, 0=>0],
                'seventh' => [5=>0, 4=>2, 3=>0, 2=>1, 1=>0, 0=>0]
            ],
            'F' => [
                'major' => [5=>1, 4=>3, 3=>3, 2=>2, 1=>1, 0=>1],
                'minor' => [5=>1, 4=>3, 3=>3, 2=>1, 1=>1, 0=>1],
                'seventh' => [5=>1, 4=>3, 3=>1, 2=>2, 1=>1, 0=>1]
            ],
            'G' => [
                'major' => [5=>3, 4=>2, 3=>0, 2=>0, 1=>0, 0=>3],
                'minor' => [5=>3, 4=>5, 3=>5, 2=>3, 1=>3, 0=>3],
                'seventh' => [5=>3, 4=>2, 3=>0, 2=>0, 1=>0, 0=>1]
            ],
            'A' => [
                'major' => [5=>0, 4=>2, 3=>2, 2=>2, 1=>0],
                'minor' => [5=>0, 4=>2, 3=>2, 2=>1, 1=>0],
                'seventh' => [5=>0, 4=>2, 3=>0, 2=>2, 1=>0]
            ],
            'B' => [
                'major' => [5=>2, 4=>4, 3=>4, 2=>4, 1=>2],
                'minor' => [5=>2, 4=>4, 3=>4, 2=>3, 1=>2],
                'seventh' => [5=>2, 4=>1, 3=>2, 2=>0, 1=>2]
            ]
        ];
    }

    private function getString($pos, $str) {
        if (isset($pos[$str])) {
            return $pos[$str] == 0 ? 'o' : (string)$pos[$str];
        }
        return 'x';
    }

    private function printFretboard($pos) {
        $labels = ['E', 'A', 'D', 'G', 'B', 'e'];
        echo "\n   e B G D A E\n";
        echo "   ────────────\n";
        for ($fret = 0; $fret < 6; $fret++) {
            $line = $fret == 0 ? '  ' : "{$fret} ";
            $line .= ' |';
            for ($str = 0; $str < 6; $str++) {
                if (isset($pos[$str]) && $pos[$str] == $fret) {
                    $line .= ' ● |';
                } elseif (isset($pos[$str]) && $pos[$str] < $fret) {
                    $line .= '   |';
                } else {
                    $line .= '   |';
                }
            }
            echo $line . "\n";
        }
        echo "\nАппликатура:\n";
        for ($str = 0; $str < 6; $str++) {
            if (isset($pos[$str])) {
                if ($pos[$str] == 0) {
                    echo "  {$labels[$str]}: открытая\n";
                } else {
                    echo "  {$labels[$str]}: {$pos[$str]}-й лад\n";
                }
            }
        }
    }

    public function getChord($root, $type) {
        if (isset($this->chords[$root]) && isset($this->chords[$root][$type])) {
            return $this->chords[$root][$type];
        }
        return null;
    }

    public function listChords() {
        echo "Доступные аккорды:\n";
        $roots = array_keys($this->chords);
        sort($roots);
        foreach ($roots as $root) {
            $types = implode(', ', array_keys($this->chords[$root]));
            echo "  $root: $types\n";
        }
    }
}

function main() {
    $chordGen = new GuitarChord();
    echo "🎸 Guitar Chord Generator (PHP)\n";
    echo "Введите аккорд в формате: <корень> <тип>\n";
    echo "Пример: C major, D minor, E seventh\n";
    echo "Или введите 'list' для просмотра всех аккордов\n";
    echo "Введите 'exit' для выхода\n";

    while (true) {
        echo "\n> ";
        $input = trim(fgets(STDIN));
        if ($input == 'exit' || $input == 'quit') {
            echo "До свидания!\n";
            break;
        }
        if ($input == 'list') {
            $chordGen->listChords();
            continue;
        }

        $parts = explode(' ', $input);
        if (count($parts) < 2) {
            echo "Неверный формат. Используйте: <корень> <тип>\n";
            continue;
        }

        $root = strtoupper($parts[0]);
        $type = strtolower($parts[1]);

        $pos = $chordGen->getChord($root, $type);
        if ($pos === null) {
            echo "❌ Аккорд $root $type не найден.\n";
            echo "Используйте 'list' для просмотра всех доступных аккордов.\n";
            continue;
        }

        echo "\n🎸 Аккорд: $root ($type)\n";
        $chordGen->printFretboard($pos);
    }
}

main();
?>
