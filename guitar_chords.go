// guitar_chords.go — Go версия

package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type GuitarChord struct {
	chords map[string]map[string]map[int]int
}

func NewGuitarChord() *GuitarChord {
	gc := &GuitarChord{
		chords: make(map[string]map[string]map[int]int),
	}
	gc.initChords()
	return gc
}

func (gc *GuitarChord) initChords() {
	// C
	gc.chords["C"] = make(map[string]map[int]int)
	gc.chords["C"]["major"] = map[int]int{5: 3, 4: 2, 3: 0, 2: 1, 1: 0}
	gc.chords["C"]["minor"] = map[int]int{5: 3, 4: 5, 3: 5, 2: 4, 1: 3}
	gc.chords["C"]["seventh"] = map[int]int{5: 3, 4: 2, 3: 3, 2: 1, 1: 0}

	// D
	gc.chords["D"] = make(map[string]map[int]int)
	gc.chords["D"]["major"] = map[int]int{4: 0, 3: 2, 2: 3, 1: 2}
	gc.chords["D"]["minor"] = map[int]int{4: 0, 3: 2, 2: 3, 1: 1}
	gc.chords["D"]["seventh"] = map[int]int{4: 0, 3: 2, 2: 1, 1: 2}

	// E
	gc.chords["E"] = make(map[string]map[int]int)
	gc.chords["E"]["major"] = map[int]int{5: 0, 4: 2, 3: 2, 2: 1, 1: 0, 0: 0}
	gc.chords["E"]["minor"] = map[int]int{5: 0, 4: 2, 3: 2, 2: 0, 1: 0, 0: 0}
	gc.chords["E"]["seventh"] = map[int]int{5: 0, 4: 2, 3: 0, 2: 1, 1: 0, 0: 0}

	// F
	gc.chords["F"] = make(map[string]map[int]int)
	gc.chords["F"]["major"] = map[int]int{5: 1, 4: 3, 3: 3, 2: 2, 1: 1, 0: 1}
	gc.chords["F"]["minor"] = map[int]int{5: 1, 4: 3, 3: 3, 2: 1, 1: 1, 0: 1}
	gc.chords["F"]["seventh"] = map[int]int{5: 1, 4: 3, 3: 1, 2: 2, 1: 1, 0: 1}

	// G
	gc.chords["G"] = make(map[string]map[int]int)
	gc.chords["G"]["major"] = map[int]int{5: 3, 4: 2, 3: 0, 2: 0, 1: 0, 0: 3}
	gc.chords["G"]["minor"] = map[int]int{5: 3, 4: 5, 3: 5, 2: 3, 1: 3, 0: 3}
	gc.chords["G"]["seventh"] = map[int]int{5: 3, 4: 2, 3: 0, 2: 0, 1: 0, 0: 1}

	// A
	gc.chords["A"] = make(map[string]map[int]int)
	gc.chords["A"]["major"] = map[int]int{5: 0, 4: 2, 3: 2, 2: 2, 1: 0}
	gc.chords["A"]["minor"] = map[int]int{5: 0, 4: 2, 3: 2, 2: 1, 1: 0}
	gc.chords["A"]["seventh"] = map[int]int{5: 0, 4: 2, 3: 0, 2: 2, 1: 0}

	// B
	gc.chords["B"] = make(map[string]map[int]int)
	gc.chords["B"]["major"] = map[int]int{5: 2, 4: 4, 3: 4, 2: 4, 1: 2}
	gc.chords["B"]["minor"] = map[int]int{5: 2, 4: 4, 3: 4, 2: 3, 1: 2}
	gc.chords["B"]["seventh"] = map[int]int{5: 2, 4: 1, 3: 2, 2: 0, 1: 2}
}

func (gc *GuitarChord) getString(pos map[int]int, str int) string {
	if val, ok := pos[str]; ok {
		if val == 0 {
			return "o"
		}
		return fmt.Sprintf("%d", val)
	}
	return "x"
}

func (gc *GuitarChord) printFretboard(pos map[int]int) {
	labels := []string{"E", "A", "D", "G", "B", "e"}
	fmt.Println("\n   e B G D A E")
	fmt.Println("   ────────────")
	for fret := 0; fret < 6; fret++ {
		line := ""
		if fret == 0 {
			line = "  "
		} else {
			line = fmt.Sprintf("%d ", fret)
		}
		line += " |"
		for str := 0; str < 6; str++ {
			if val, ok := pos[str]; ok && val == fret {
				line += " ● |"
			} else if val, ok := pos[str]; ok && val < fret {
				line += "   |"
			} else {
				line += "   |"
			}
		}
		fmt.Println(line)
	}
	fmt.Println("\nАппликатура:")
	for str := 0; str < 6; str++ {
		if val, ok := pos[str]; ok {
			if val == 0 {
				fmt.Printf("  %s: открытая\n", labels[str])
			} else {
				fmt.Printf("  %s: %d-й лад\n", labels[str], val)
			}
		}
	}
}

func (gc *GuitarChord) getChord(root, chordType string) map[int]int {
	if chordMap, ok := gc.chords[root]; ok {
		if pos, ok := chordMap[chordType]; ok {
			return pos
		}
	}
	return nil
}

func (gc *GuitarChord) listChords() {
	fmt.Println("Доступные аккорды:")
	for root := range gc.chords {
		types := []string{}
		for t := range gc.chords[root] {
			types = append(types, t)
		}
		fmt.Printf("  %s: %s\n", root, strings.Join(types, ", "))
	}
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	chordGen := NewGuitarChord()

	fmt.Println("🎸 Guitar Chord Generator (Go)")
	fmt.Println("Введите аккорд в формате: <корень> <тип>")
	fmt.Println("Пример: C major, D minor, E seventh")
	fmt.Println("Или введите 'list' для просмотра всех аккордов")

	for {
		fmt.Print("\n> ")
		input, _ := reader.ReadString('\n')
		input = strings.TrimSpace(input)

		if input == "exit" || input == "quit" {
			break
		}
		if input == "list" {
			chordGen.listChords()
			continue
		}

		parts := strings.Split(input, " ")
		if len(parts) < 2 {
			fmt.Println("Неверный формат. Используйте: <корень> <тип>")
			continue
		}
		root := strings.ToUpper(parts[0])
		chordType := strings.ToLower(parts[1])

		pos := chordGen.getChord(root, chordType)
		if pos == nil {
			fmt.Printf("❌ Аккорд %s %s не найден.\n", root, chordType)
			fmt.Println("Используйте 'list' для просмотра всех доступных аккордов.")
			continue
		}
		fmt.Printf("\n🎸 Аккорд: %s (%s)\n", root, chordType)
		chordGen.printFretboard(pos)
	}
}
