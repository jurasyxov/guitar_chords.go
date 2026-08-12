

### 1. `guitar_chords.py` (Python)

```python
# guitar_chords.py — Python версия

import sys
import argparse
from colorama import init, Fore, Style

init(autoreset=True)

class GuitarChord:
    def __init__(self):
        self.chords = {
            'C': {'major': self._c_major, 'minor': self._c_minor, 'seventh': self._c_seventh},
            'D': {'major': self._d_major, 'minor': self._d_minor, 'seventh': self._d_seventh},
            'E': {'major': self._e_major, 'minor': self._e_minor, 'seventh': self._e_seventh},
            'F': {'major': self._f_major, 'minor': self._f_minor, 'seventh': self._f_seventh},
            'G': {'major': self._g_major, 'minor': self._g_minor, 'seventh': self._g_seventh},
            'A': {'major': self._a_major, 'minor': self._a_minor, 'seventh': self._a_seventh},
            'B': {'major': self._b_major, 'minor': self._b_minor, 'seventh': self._b_seventh},
        }

    def _get_strings(self, positions):
        """Возвращает схему грифа для 6 струн"""
        strings = []
        for i in range(6):
            fret = positions.get(i, None)
            if fret is None:
                strings.append('x')  # не играется
            elif fret == 0:
                strings.append('o')  # открытая
            else:
                strings.append(str(fret))
        return strings

    def _print_fretboard(self, positions):
        """Печать грифа с аппликатурой"""
        strings = self._get_strings(positions)
        print("\n" + Fore.CYAN + "   e B G D A E")
        print("   ────────────")
        for fret in range(6):
            line = f"{fret+1 if fret > 0 else ' '} |"
            for i in range(6):
                if positions.get(i) == fret:
                    line += f" ● |"
                elif positions.get(i) is not None and positions.get(i) < fret:
                    line += "   |"
                else:
                    line += "   |"
            print(line)
        # Показываем открытые/зажатые строки
        labels = ['E', 'A', 'D', 'G', 'B', 'e']
        print("\n" + Fore.YELLOW + "Аппликатура:")
        for i, pos in positions.items():
            if pos == 0:
                print(f"  {labels[i]}: открытая")
            else:
                print(f"  {labels[i]}: {pos}-й лад")

    # --- Аккорды C ---
    def _c_major(self):
        # C: x-3-2-0-1-0
        return {5: 3, 4: 2, 3: 0, 2: 1, 1: 0}

    def _c_minor(self):
        # Cm: x-3-5-5-4-3
        return {5: 3, 4: 5, 3: 5, 2: 4, 1: 3}

    def _c_seventh(self):
        # C7: x-3-2-3-1-0
        return {5: 3, 4: 2, 3: 3, 2: 1, 1: 0}

    # --- Аккорды D ---
    def _d_major(self):
        # D: x-x-0-2-3-2
        return {4: 0, 3: 2, 2: 3, 1: 2}

    def _d_minor(self):
        # Dm: x-x-0-2-3-1
        return {4: 0, 3: 2, 2: 3, 1: 1}

    def _d_seventh(self):
        # D7: x-x-0-2-1-2
        return {4: 0, 3: 2, 2: 1, 1: 2}

    # --- Аккорды E ---
    def _e_major(self):
        # E: 0-2-2-1-0-0
        return {5: 0, 4: 2, 3: 2, 2: 1, 1: 0, 0: 0}

    def _e_minor(self):
        # Em: 0-2-2-0-0-0
        return {5: 0, 4: 2, 3: 2, 2: 0, 1: 0, 0: 0}

    def _e_seventh(self):
        # E7: 0-2-0-1-0-0
        return {5: 0, 4: 2, 3: 0, 2: 1, 1: 0, 0: 0}

    # --- Аккорды F ---
    def _f_major(self):
        # F: 1-3-3-2-1-1 (барре)
        return {5: 1, 4: 3, 3: 3, 2: 2, 1: 1, 0: 1}

    def _f_minor(self):
        # Fm: 1-3-3-1-1-1
        return {5: 1, 4: 3, 3: 3, 2: 1, 1: 1, 0: 1}

    def _f_seventh(self):
        # F7: 1-3-1-2-1-1
        return {5: 1, 4: 3, 3: 1, 2: 2, 1: 1, 0: 1}

    # --- Аккорды G ---
    def _g_major(self):
        # G: 3-2-0-0-0-3
        return {5: 3, 4: 2, 3: 0, 2: 0, 1: 0, 0: 3}

    def _g_minor(self):
        # Gm: 3-5-5-3-3-3
        return {5: 3, 4: 5, 3: 5, 2: 3, 1: 3, 0: 3}

    def _g_seventh(self):
        # G7: 3-2-0-0-0-1
        return {5: 3, 4: 2, 3: 0, 2: 0, 1: 0, 0: 1}

    # --- Аккорды A ---
    def _a_major(self):
        # A: x-0-2-2-2-0
        return {5: 0, 4: 2, 3: 2, 2: 2, 1: 0}

    def _a_minor(self):
        # Am: x-0-2-2-1-0
        return {5: 0, 4: 2, 3: 2, 2: 1, 1: 0}

    def _a_seventh(self):
        # A7: x-0-2-0-2-0
        return {5: 0, 4: 2, 3: 0, 2: 2, 1: 0}

    # --- Аккорды B ---
    def _b_major(self):
        # B: x-2-4-4-4-2
        return {5: 2, 4: 4, 3: 4, 2: 4, 1: 2}

    def _b_minor(self):
        # Bm: x-2-4-4-3-2
        return {5: 2, 4: 4, 3: 4, 2: 3, 1: 2}

    def _b_seventh(self):
        # B7: x-2-1-2-0-2
        return {5: 2, 4: 1, 3: 2, 2: 0, 1: 2}

    def get_chord(self, root, chord_type):
        if root in self.chords and chord_type in self.chords[root]:
            return self.chords[root][chord_type]()
        return None

    def list_chords(self):
        print(Fore.CYAN + "Доступные аккорды:")
        for root in sorted(self.chords.keys()):
            types = list(self.chords[root].keys())
            print(f"  {root}: {', '.join(types)}")

def main():
    parser = argparse.ArgumentParser(description='Guitar Chord Generator')
    parser.add_argument('--chord', default='C', help='Корень аккорда (C, D, E, F, G, A, B)')
    parser.add_argument('--type', default='major', help='Тип аккорда (major, minor, seventh)')
    parser.add_argument('--list', action='store_true', help='Показать все доступные аккорды')
    args = parser.parse_args()

    generator = GuitarChord()

    if args.list:
        generator.list_chords()
        sys.exit(0)

    chord_func = generator.get_chord(args.chord, args.type)
    if chord_func is None:
        print(Fore.RED + f"❌ Аккорд {args.chord} {args.type} не найден.")
        print(f"Используйте --list для просмотра всех доступных аккордов.")
        sys.exit(1)

    positions = chord_func
    print(f"\n{Fore.GREEN}🎸 Аккорд: {args.chord} ({args.type})")
    generator._print_fretboard(positions)

if __name__ == "__main__":
    main()
