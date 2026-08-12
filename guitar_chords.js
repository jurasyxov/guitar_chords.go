// guitar_chords.js — JavaScript версия

const readline = require('readline');

class GuitarChord {
    constructor() {
        this.chords = {
            'C': {
                'major': {5:3, 4:2, 3:0, 2:1, 1:0},
                'minor': {5:3, 4:5, 3:5, 2:4, 1:3},
                'seventh': {5:3, 4:2, 3:3, 2:1, 1:0}
            },
            'D': {
                'major': {4:0, 3:2, 2:3, 1:2},
                'minor': {4:0, 3:2, 2:3, 1:1},
                'seventh': {4:0, 3:2, 2:1, 1:2}
            },
            'E': {
                'major': {5:0, 4:2, 3:2, 2:1, 1:0, 0:0},
                'minor': {5:0, 4:2, 3:2, 2:0, 1:0, 0:0},
                'seventh': {5:0, 4:2, 3:0, 2:1, 1:0, 0:0}
            },
            'F': {
                'major': {5:1, 4:3, 3:3, 2:2, 1:1, 0:1},
                'minor': {5:1, 4:3, 3:3, 2:1, 1:1, 0:1},
                'seventh': {5:1, 4:3, 3:1, 2:2, 1:1, 0:1}
            },
            'G': {
                'major': {5:3, 4:2, 3:0, 2:0, 1:0, 0:3},
                'minor': {5:3, 4:5, 3:5, 2:3, 1:3, 0:3},
                'seventh': {5:3, 4:2, 3:0, 2:0, 1:0, 0:1}
            },
            'A': {
                'major': {5:0, 4:2, 3:2, 2:2, 1:0},
                'minor': {5:0, 4:2, 3:2, 2:1, 1:0},
                'seventh': {5:0, 4:2, 3:0, 2:2, 1:0}
            },
            'B': {
                'major': {5:2, 4:4, 3:4, 2:4, 1:2},
                'minor': {5:2, 4:4, 3:4, 2:3, 1:2},
                'seventh': {5:2, 4:1, 3:2, 2:0, 1:2}
            }
        };
    }

    getString(pos, str) {
        if (pos.hasOwnProperty(str)) {
            return pos[str] === 0 ? 'o' : String(pos[str]);
        }
        return 'x';
    }

    printFretboard(pos) {
        const labels = ['E', 'A', 'D', 'G', 'B', 'e'];
        console.log('\n   e B G D A E');
        console.log('   ────────────');
        for (let fret = 0; fret < 6; fret++) {
            let line = fret === 0 ? '  ' : `${fret} `;
            line += ' |';
            for (let str = 0; str < 6; str++) {
                if (pos.hasOwnProperty(str) && pos[str] === fret) {
                    line += ' ● |';
                } else if (pos.hasOwnProperty(str) && pos[str] < fret) {
                    line += '   |';
                } else {
                    line += '   |';
                }
            }
            console.log(line);
        }
        console.log('\nАппликатура:');
        for (let str = 0; str < 6; str++) {
            if (pos.hasOwnProperty(str)) {
                if (pos[str] === 0) {
                    console.log(`  ${labels[str]}: открытая`);
                } else {
                    console.log(`  ${labels[str]}: ${pos[str]}-й лад`);
                }
            }
        }
    }

    getChord(root, type) {
        if (this.chords.hasOwnProperty(root) && this.chords[root].hasOwnProperty(type)) {
            return this.chords[root][type];
        }
        return null;
    }

    listChords() {
        console.log('Доступные аккорды:');
        for (const root of Object.keys(this.chords).sort()) {
            const types = Object.keys(this.chords[root]);
            console.log(`  ${root}: ${types.join(', ')}`);
        }
    }
}

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

const chordGen = new GuitarChord();

console.log('🎸 Guitar Chord Generator (JavaScript)');
console.log('Введите аккорд в формате: <корень> <тип>');
console.log('Пример: C major, D minor, E seventh');
console.log('Или введите "list" для просмотра всех аккордов');
console.log('Введите "exit" для выхода');

rl.setPrompt('> ');
rl.prompt();

rl.on('line', (input) => {
    input = input.trim();
    if (input === 'exit' || input === 'quit') {
        rl.close();
        return;
    }
    if (input === 'list') {
        chordGen.listChords();
        rl.prompt();
        return;
    }

    const parts = input.split(' ');
    if (parts.length < 2) {
        console.log('Неверный формат. Используйте: <корень> <тип>');
        rl.prompt();
        return;
    }

    const root = parts[0].toUpperCase();
    const type = parts[1].toLowerCase();

    const pos = chordGen.getChord(root, type);
    if (!pos) {
        console.log(`❌ Аккорд ${root} ${type} не найден.`);
        console.log('Используйте "list" для просмотра всех доступных аккордов.');
        rl.prompt();
        return;
    }

    console.log(`\n🎸 Аккорд: ${root} (${type})`);
    chordGen.printFretboard(pos);
    rl.prompt();
});

rl.on('close', () => {
    console.log('\nДо свидания!');
    process.exit(0);
});
