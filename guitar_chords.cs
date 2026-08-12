// guitar_chords.cs — C# версия

using System;
using System.Collections.Generic;
using System.Linq;

class GuitarChord
{
    private Dictionary<string, Dictionary<string, Dictionary<int, int>>> chords;

    public GuitarChord()
    {
        chords = new Dictionary<string, Dictionary<string, Dictionary<int, int>>>();
        InitChords();
    }

    private void InitChords()
    {
        // C
        var c = new Dictionary<string, Dictionary<int, int>>();
        c["major"] = new Dictionary<int, int> { {5, 3}, {4, 2}, {3, 0}, {2, 1}, {1, 0} };
        c["minor"] = new Dictionary<int, int> { {5, 3}, {4, 5}, {3, 5}, {2, 4}, {1, 3} };
        c["seventh"] = new Dictionary<int, int> { {5, 3}, {4, 2}, {3, 3}, {2, 1}, {1, 0} };
        chords["C"] = c;

        // D
        var d = new Dictionary<string, Dictionary<int, int>>();
        d["major"] = new Dictionary<int, int> { {4, 0}, {3, 2}, {2, 3}, {1, 2} };
        d["minor"] = new Dictionary<int, int> { {4, 0}, {3, 2}, {2, 3}, {1, 1} };
        d["seventh"] = new Dictionary<int, int> { {4, 0}, {3, 2}, {2, 1}, {1, 2} };
        chords["D"] = d;

        // E
        var e = new Dictionary<string, Dictionary<int, int>>();
        e["major"] = new Dictionary<int, int> { {5, 0}, {4, 2}, {3, 2}, {2, 1}, {1, 0}, {0, 0} };
        e["minor"] = new Dictionary<int, int> { {5, 0}, {4, 2}, {3, 2}, {2, 0}, {1, 0}, {0, 0} };
        e["seventh"] = new Dictionary<int, int> { {5, 0}, {4, 2}, {3, 0}, {2, 1}, {1, 0}, {0, 0} };
        chords["E"] = e;

        // F
        var f = new Dictionary<string, Dictionary<int, int>>();
        f["major"] = new Dictionary<int, int> { {5, 1}, {4, 3}, {3, 3}, {2, 2}, {1, 1}, {0, 1} };
        f["minor"] = new Dictionary<int, int> { {5, 1}, {4, 3}, {3, 3}, {2, 1}, {1, 1}, {0, 1} };
        f["seventh"] = new Dictionary<int, int> { {5, 1}, {4, 3}, {3, 1}, {2, 2}, {1, 1}, {0, 1} };
        chords["F"] = f;

        // G
        var g = new Dictionary<string, Dictionary<int, int>>();
        g["major"] = new Dictionary<int, int> { {5, 3}, {4, 2}, {3, 0}, {2, 0}, {1, 0}, {0, 3} };
        g["minor"] = new Dictionary<int, int> { {5, 3}, {4, 5}, {3, 5}, {2, 3}, {1, 3}, {0, 3} };
        g["seventh"] = new Dictionary<int, int> { {5, 3}, {4, 2}, {3, 0}, {2, 0}, {1, 0}, {0, 1} };
        chords["G"] = g;

        // A
        var a = new Dictionary<string, Dictionary<int, int>>();
        a["major"] = new Dictionary<int, int> { {5, 0}, {4, 2}, {3, 2}, {2, 2}, {1, 0} };
        a["minor"] = new Dictionary<int, int> { {5, 0}, {4, 2}, {3, 2}, {2, 1}, {1, 0} };
        a["seventh"] = new Dictionary<int, int> { {5, 0}, {4, 2}, {3, 0}, {2, 2}, {1, 0} };
        chords["A"] = a;

        // B
        var b = new Dictionary<string, Dictionary<int, int>>();
        b["major"] = new Dictionary<int, int> { {5, 2}, {4, 4}, {3, 4}, {2, 4}, {1, 2} };
        b["minor"] = new Dictionary<int, int> { {5, 2}, {4, 4}, {3, 4}, {2, 3}, {1, 2} };
        b["seventh"] = new Dictionary<int, int> { {5, 2}, {4, 1}, {3, 2}, {2, 0}, {1, 2} };
        chords["B"] = b;
    }

    private string GetString(Dictionary<int, int> pos, int str)
    {
        if (pos.ContainsKey(str))
            return pos[str] == 0 ? "o" : pos[str].ToString();
        return "x";
    }

    private void PrintFretboard(Dictionary<int, int> pos)
    {
        string[] labels = { "E", "A", "D", "G", "B", "e" };
        Console.WriteLine("\n   e B G D A E");
        Console.WriteLine("   ────────────");
        for (int fret = 0; fret < 6; fret++)
        {
            string line = fret == 0 ? "  " : $"{fret} ";
            line += " |";
            for (int str = 0; str < 6; str++)
            {
                if (pos.ContainsKey(str) && pos[str] == fret)
                    line += " ● |";
                else if (pos.ContainsKey(str) && pos[str] < fret)
                    line += "   |";
                else
                    line += "   |";
            }
            Console.WriteLine(line);
        }
        Console.WriteLine("\nАппликатура:");
        for (int str = 0; str < 6; str++)
        {
            if (pos.ContainsKey(str))
            {
                if (pos[str] == 0)
                    Console.WriteLine($"  {labels[str]}: открытая");
                else
                    Console.WriteLine($"  {labels[str]}: {pos[str]}-й лад");
            }
        }
    }

    public Dictionary<int, int> GetChord(string root, string type)
    {
        if (chords.ContainsKey(root) && chords[root].ContainsKey(type))
            return chords[root][type];
        return null;
    }

    public void ListChords()
    {
        Console.WriteLine("Доступные аккорды:");
        foreach (var root in chords.Keys.OrderBy(k => k))
        {
            var types = string.Join(", ", chords[root].Keys);
            Console.WriteLine($"  {root}: {types}");
        }
    }

    public static void Main()
    {
        var chordGen = new GuitarChord();
        Console.WriteLine("🎸 Guitar Chord Generator (C#)");
        Console.WriteLine("Введите аккорд в формате: <корень> <тип>");
        Console.WriteLine("Пример: C major, D minor, E seventh");
        Console.WriteLine("Или введите 'list' для просмотра всех аккордов");
        Console.WriteLine("Введите 'exit' для выхода");

        while (true)
        {
            Console.Write("\n> ");
            string input = Console.ReadLine().Trim();
            if (input == "exit" || input == "quit")
            {
                Console.WriteLine("До свидания!");
                break;
            }
            if (input == "list")
            {
                chordGen.ListChords();
                continue;
            }

            string[] parts = input.Split(' ');
            if (parts.Length < 2)
            {
                Console.WriteLine("Неверный формат. Используйте: <корень> <тип>");
                continue;
            }

            string root = parts[0].ToUpper();
            string type = parts[1].ToLower();

            var pos = chordGen.GetChord(root, type);
            if (pos == null)
            {
                Console.WriteLine($"❌ Аккорд {root} {type} не найден.");
                Console.WriteLine("Используйте 'list' для просмотра всех доступных аккордов.");
                continue;
            }
            Console.WriteLine($"\n🎸 Аккорд: {root} ({type})");
            chordGen.PrintFretboard(pos);
        }
    }
}
