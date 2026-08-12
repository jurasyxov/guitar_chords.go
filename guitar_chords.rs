// guitar_chords.rs — Rust версия

use std::collections::HashMap;
use std::io::{self, Write};

struct GuitarChord {
    chords: HashMap<String, HashMap<String, HashMap<usize, usize>>>,
}

impl GuitarChord {
    fn new() -> Self {
        let mut chords = HashMap::new();

        // C
        let mut c = HashMap::new();
        c.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 3); m.insert(4, 2); m.insert(3, 0); m.insert(2, 1); m.insert(1, 0);
            m
        });
        c.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 3); m.insert(4, 5); m.insert(3, 5); m.insert(2, 4); m.insert(1, 3);
            m
        });
        c.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 3); m.insert(4, 2); m.insert(3, 3); m.insert(2, 1); m.insert(1, 0);
            m
        });
        chords.insert("C".to_string(), c);

        // D
        let mut d = HashMap::new();
        d.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(4, 0); m.insert(3, 2); m.insert(2, 3); m.insert(1, 2);
            m
        });
        d.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(4, 0); m.insert(3, 2); m.insert(2, 3); m.insert(1, 1);
            m
        });
        d.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(4, 0); m.insert(3, 2); m.insert(2, 1); m.insert(1, 2);
            m
        });
        chords.insert("D".to_string(), d);

        // E
        let mut e = HashMap::new();
        e.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 0); m.insert(4, 2); m.insert(3, 2); m.insert(2, 1); m.insert(1, 0); m.insert(0, 0);
            m
        });
        e.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 0); m.insert(4, 2); m.insert(3, 2); m.insert(2, 0); m.insert(1, 0); m.insert(0, 0);
            m
        });
        e.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 0); m.insert(4, 2); m.insert(3, 0); m.insert(2, 1); m.insert(1, 0); m.insert(0, 0);
            m
        });
        chords.insert("E".to_string(), e);

        // F
        let mut f = HashMap::new();
        f.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 1); m.insert(4, 3); m.insert(3, 3); m.insert(2, 2); m.insert(1, 1); m.insert(0, 1);
            m
        });
        f.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 1); m.insert(4, 3); m.insert(3, 3); m.insert(2, 1); m.insert(1, 1); m.insert(0, 1);
            m
        });
        f.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 1); m.insert(4, 3); m.insert(3, 1); m.insert(2, 2); m.insert(1, 1); m.insert(0, 1);
            m
        });
        chords.insert("F".to_string(), f);

        // G
        let mut g = HashMap::new();
        g.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 3); m.insert(4, 2); m.insert(3, 0); m.insert(2, 0); m.insert(1, 0); m.insert(0, 3);
            m
        });
        g.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 3); m.insert(4, 5); m.insert(3, 5); m.insert(2, 3); m.insert(1, 3); m.insert(0, 3);
            m
        });
        g.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 3); m.insert(4, 2); m.insert(3, 0); m.insert(2, 0); m.insert(1, 0); m.insert(0, 1);
            m
        });
        chords.insert("G".to_string(), g);

        // A
        let mut a = HashMap::new();
        a.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 0); m.insert(4, 2); m.insert(3, 2); m.insert(2, 2); m.insert(1, 0);
            m
        });
        a.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 0); m.insert(4, 2); m.insert(3, 2); m.insert(2, 1); m.insert(1, 0);
            m
        });
        a.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 0); m.insert(4, 2); m.insert(3, 0); m.insert(2, 2); m.insert(1, 0);
            m
        });
        chords.insert("A".to_string(), a);

        // B
        let mut b = HashMap::new();
        b.insert("major".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 2); m.insert(4, 4); m.insert(3, 4); m.insert(2, 4); m.insert(1, 2);
            m
        });
        b.insert("minor".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 2); m.insert(4, 4); m.insert(3, 4); m.insert(2, 3); m.insert(1, 2);
            m
        });
        b.insert("seventh".to_string(), {
            let mut m = HashMap::new();
            m.insert(5, 2); m.insert(4, 1); m.insert(3, 2); m.insert(2, 0); m.insert(1, 2);
            m
        });
        chords.insert("B".to_string(), b);

        GuitarChord { chords }
    }

    fn get_string(&self, pos: &HashMap<usize, usize>, str: usize) -> String {
        if let Some(&val) = pos.get(&str) {
            if val == 0 { "o".to_string() } else { val.to_string() }
        } else {
            "x".to_string()
        }
    }

    fn print_fretboard(&self, pos: &HashMap<usize, usize>) {
        let labels = ["E", "A", "D", "G", "B", "e"];
        println!("\n   e B G D A E");
        println!("   ────────────");
        for fret in 0..6 {
            let mut line = if fret == 0 { "  ".to_string() } else { format!("{} ", fret) };
            line.push_str(" |");
            for str in 0..6 {
                if let Some(&val) = pos.get(&str) {
                    if val == fret {
                        line.push_str(" ● |");
                    } else if val < fret {
                        line.push_str("   |");
                    } else {
                        line.push_str("   |");
                    }
                } else {
                    line.push_str("   |");
                }
            }
            println!("{}", line);
        }
        println!("\nАппликатура:");
        for str in 0..6 {
            if let Some(&val) = pos.get(&str) {
                if val == 0 {
                    println!("  {}: открытая", labels[str]);
                } else {
                    println!("  {}: {}-й лад", labels[str], val);
                }
            }
        }
    }

    fn get_chord(&self, root: &str, typ: &str) -> Option<HashMap<usize, usize>> {
        if let Some(root_map) = self.chords.get(root) {
            if let Some(pos) = root_map.get(typ) {
                return Some(pos.clone());
            }
        }
        None
    }

    fn list_chords(&self) {
        println!("Доступные аккорды:");
        let mut roots: Vec<_> = self.chords.keys().collect();
        roots.sort();
        for root in roots {
            let types: Vec<_> = self.chords[root].keys().collect();
            println!("  {}: {}", root, types.join(", "));
        }
    }
}

fn main() {
    let chord_gen = GuitarChord::new();
    println!("🎸 Guitar Chord Generator (Rust)");
    println!("Введите аккорд в формате: <корень> <тип>");
    println!("Пример: C major, D minor, E seventh");
    println!("Или введите 'list' для просмотра всех аккордов");
    println!("Введите 'exit' для выхода");

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" || input == "quit" {
            println!("До свидания!");
            break;
        }
        if input == "list" {
            chord_gen.list_chords();
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 2 {
            println!("Неверный формат. Используйте: <корень> <тип>");
            continue;
        }

        let root = parts[0].to_uppercase();
        let typ = parts[1].to_lowercase();

        if let Some(pos) = chord_gen.get_chord(&root, &typ) {
            println!("\n🎸 Аккорд: {} ({})", root, typ);
            chord_gen.print_fretboard(&pos);
        } else {
            println!("❌ Аккорд {} {} не найден.", root, typ);
            println!("Используйте 'list' для просмотра всех доступных аккордов.");
        }
    }
}
