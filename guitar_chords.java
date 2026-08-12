// guitar_chords.java — Java версия

import java.io.*;
import java.util.*;

class GuitarChord {
    private Map<String, Map<String, Map<Integer, Integer>>> chords;

    public GuitarChord() {
        chords = new HashMap<>();
        initChords();
    }

    private void initChords() {
        // C
        Map<String, Map<Integer, Integer>> c = new HashMap<>();
        c.put("major", new HashMap<Integer, Integer>() {{
            put(5, 3); put(4, 2); put(3, 0); put(2, 1); put(1, 0);
        }});
        c.put("minor", new HashMap<Integer, Integer>() {{
            put(5, 3); put(4, 5); put(3, 5); put(2, 4); put(1, 3);
        }});
        c.put("seventh", new HashMap<Integer, Integer>() {{
            put(5, 3); put(4, 2); put(3, 3); put(2, 1); put(1, 0);
        }});
        chords.put("C", c);

        // D
        Map<String, Map<Integer, Integer>> d = new HashMap<>();
        d.put("major", new HashMap<Integer, Integer>() {{ put(4, 0); put(3, 2); put(2, 3); put(1, 2); }});
        d.put("minor", new HashMap<Integer, Integer>() {{ put(4, 0); put(3, 2); put(2, 3); put(1, 1); }});
        d.put("seventh", new HashMap<Integer, Integer>() {{ put(4, 0); put(3, 2); put(2, 1); put(1, 2); }});
        chords.put("D", d);

        // E
        Map<String, Map<Integer, Integer>> e = new HashMap<>();
        e.put("major", new HashMap<Integer, Integer>() {{ put(5, 0); put(4, 2); put(3, 2); put(2, 1); put(1, 0); put(0, 0); }});
        e.put("minor", new HashMap<Integer, Integer>() {{ put(5, 0); put(4, 2); put(3, 2); put(2, 0); put(1, 0); put(0, 0); }});
        e.put("seventh", new HashMap<Integer, Integer>() {{ put(5, 0); put(4, 2); put(3, 0); put(2, 1); put(1, 0); put(0, 0); }});
        chords.put("E", e);

        // F
        Map<String, Map<Integer, Integer>> f = new HashMap<>();
        f.put("major", new HashMap<Integer, Integer>() {{ put(5, 1); put(4, 3); put(3, 3); put(2, 2); put(1, 1); put(0, 1); }});
        f.put("minor", new HashMap<Integer, Integer>() {{ put(5, 1); put(4, 3); put(3, 3); put(2, 1); put(1, 1); put(0, 1); }});
        f.put("seventh", new HashMap<Integer, Integer>() {{ put(5, 1); put(4, 3); put(3, 1); put(2, 2); put(1, 1); put(0, 1); }});
        chords.put("F", f);

        // G
        Map<String, Map<Integer, Integer>> g = new HashMap<>();
        g.put("major", new HashMap<Integer, Integer>() {{ put(5, 3); put(4, 2); put(3, 0); put(2, 0); put(1, 0); put(0, 3); }});
        g.put("minor", new HashMap<Integer, Integer>() {{ put(5, 3); put(4, 5); put(3, 5); put(2, 3); put(1, 3); put(0, 3); }});
        g.put("seventh", new HashMap<Integer, Integer>() {{ put(5, 3); put(4, 2); put(3, 0); put(2, 0); put(1, 0); put(0, 1); }});
        chords.put("G", g);

        // A
        Map<String, Map<Integer, Integer>> a = new HashMap<>();
        a.put("major", new HashMap<Integer, Integer>() {{ put(5, 0); put(4, 2); put(3, 2); put(2, 2); put(1, 0); }});
        a.put("minor", new HashMap<Integer, Integer>() {{ put(5, 0); put(4, 2); put(3, 2); put(2, 1); put(1, 0); }});
        a.put("seventh", new HashMap<Integer, Integer>() {{ put(5, 0); put(4, 2); put(3, 0); put(2, 2); put(1, 0); }});
        chords.put("A", a);

        // B
        Map<String, Map<Integer, Integer>> b = new HashMap<>();
        b.put("major", new HashMap<Integer, Integer>() {{ put(5, 2); put(4, 4); put(3, 4); put(2, 4); put(1, 2); }});
        b.put("minor", new HashMap<Integer, Integer>() {{ put(5, 2); put(4, 4); put(3, 4); put(2, 3); put(1, 2); }});
        b.put("seventh", new HashMap<Integer, Integer>() {{ put(5, 2); put(4, 1); put(3, 2); put(2, 0); put(1, 2); }});
        chords.put("B", b);
    }

    private String getString(Map<Integer, Integer> pos, int str) {
        if (pos.containsKey(str)) {
            return pos.get(str) == 0 ? "o" : String.valueOf(pos.get(str));
        }
        return "x";
    }

    private void printFretboard(Map<Integer, Integer> pos) {
        String[] labels = {"E", "A", "D", "G", "B", "e"};
        System.out.println("\n   e B G D A E");
        System.out.println("   ────────────");
        for (int fret = 0; fret < 6; fret++) {
            String line = fret == 0 ? "  " : String.valueOf(fret);
            line += " |";
            for (int str = 0; str < 6; str++) {
                if (pos.containsKey(str) && pos.get(str) == fret) {
                    line += " ● |";
                } else if (pos.containsKey(str) && pos.get(str) < fret) {
                    line += "   |";
                } else {
                    line += "   |";
                }
            }
            System.out.println(line);
        }
        System.out.println("\nАппликатура:");
        for (int str = 0; str < 6; str++) {
            if (pos.containsKey(str)) {
                if (pos.get(str) == 0) {
                    System.out.printf("  %s: открытая\n", labels[str]);
                } else {
                    System.out.printf("  %s: %d-й лад\n", labels[str], pos.get(str));
                }
            }
        }
    }

    public Map<Integer, Integer> getChord(String root, String type) {
        if (chords.containsKey(root) && chords.get(root).containsKey(type)) {
            return chords.get(root).get(type);
        }
        return null;
    }

    public void listChords() {
        System.out.println("Доступные аккорды:");
        for (String root : new TreeSet<>(chords.keySet())) {
            Set<String> types = chords.get(root).keySet();
            System.out.printf("  %s: %s\n", root, String.join(", ", types));
        }
    }

    public static void main(String[] args) throws IOException {
        GuitarChord chordGen = new GuitarChord();
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));

        System.out.println("🎸 Guitar Chord Generator (Java)");
        System.out.println("Введите аккорд в формате: <корень> <тип>");
        System.out.println("Пример: C major, D minor, E seventh");
        System.out.println("Или введите 'list' для просмотра всех аккордов");
        System.out.println("Введите 'exit' для выхода");

        while (true) {
            System.out.print("\n> ");
            String input = reader.readLine().trim();
            if (input.equals("exit") || input.equals("quit")) {
                System.out.println("До свидания!");
                break;
            }
            if (input.equals("list")) {
                chordGen.listChords();
                continue;
            }

            String[] parts = input.split(" ");
            if (parts.length < 2) {
                System.out.println("Неверный формат. Используйте: <корень> <тип>");
                continue;
            }

            String root = parts[0].toUpperCase();
            String type = parts[1].toLowerCase();

            Map<Integer, Integer> pos = chordGen.getChord(root, type);
            if (pos == null) {
                System.out.printf("❌ Аккорд %s %s не найден.\n", root, type);
                System.out.println("Используйте 'list' для просмотра всех доступных аккордов.");
                continue;
            }
            System.out.printf("\n🎸 Аккорд: %s (%s)\n", root, type);
            chordGen.printFretboard(pos);
        }
    }
}
