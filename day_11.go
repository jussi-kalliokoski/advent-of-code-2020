package main

import (
	"bytes"
	"fmt"
	"io"
	"io/ioutil"
	"os"
)

func main() {
	m, err := parseSeatMap(os.Stdin)
	if err != nil {
		panic(err)
	}
	fmt.Println(m.RowWidth, len(m.Seats)/m.RowWidth)
	scratch := [...]seatMap{m.Clone(), m.Clone()}
	firstAnswer := m.RearrangeUntilStable(scratch, func(m seatMap, x, y int, seatType seatType) seatType {
		switch seatType {
		case seatTypeEmpty:
			if m.CountAdjacentSeatsOfType(x, y, seatTypeTaken) == 0 {
				return seatTypeTaken
			}
		case seatTypeTaken:
			if m.CountAdjacentSeatsOfType(x, y, seatTypeTaken) >= 4 {
				return seatTypeEmpty
			}
		}
		return seatType
	}).CountSeatsOfType(seatTypeTaken)
	fmt.Println("first answer:", firstAnswer)
	secondAnswer := m.RearrangeUntilStable(scratch, func(m seatMap, x, y int, seatType seatType) seatType {
		switch seatType {
		case seatTypeEmpty:
			if m.CountVisibleSeatsOfType(x, y, seatTypeTaken) == 0 {
				return seatTypeTaken
			}
		case seatTypeTaken:
			if m.CountVisibleSeatsOfType(x, y, seatTypeTaken) >= 5 {
				return seatTypeEmpty
			}
		}
		return seatType
	}).CountSeatsOfType(seatTypeTaken)
	fmt.Println("second answer:", secondAnswer)
}

type seatMap struct {
	RowWidth int
	Seats    []seatType
}

func parseSeatMap(r io.Reader) (seatMap, error) {
	data, err := ioutil.ReadAll(r)
	if err != nil {
		return seatMap{}, err
	}
	seats := make([]seatType, 0, len(data))
	var rowWidth int
	for _, line := range bytes.Split(data, []byte{'\n'}) {
		rowWidth = len(line)
		for _, c := range line {
			switch c {
			case '.':
				seats = append(seats, seatTypeFloor)
			case 'L':
				seats = append(seats, seatTypeEmpty)
			case '#':
				seats = append(seats, seatTypeTaken)
			}
		}
	}
	return seatMap{rowWidth, seats}, nil
}

func (m seatMap) CountVisibleSeatsOfType(x, y int, seatType seatType) int {
	count := 0
	count += btoi(m.HasSeatVisibleInDirection(x, y, -1, -1, seatType))
	count += btoi(m.HasSeatVisibleInDirection(x, y, +0, -1, seatType))
	count += btoi(m.HasSeatVisibleInDirection(x, y, +1, -1, seatType))
	count += btoi(m.HasSeatVisibleInDirection(x, y, -1, +0, seatType))
	count += btoi(m.HasSeatVisibleInDirection(x, y, +1, +0, seatType))
	count += btoi(m.HasSeatVisibleInDirection(x, y, -1, +1, seatType))
	count += btoi(m.HasSeatVisibleInDirection(x, y, +0, +1, seatType))
	count += btoi(m.HasSeatVisibleInDirection(x, y, +1, +1, seatType))
	return count
}

func (m seatMap) CountAdjacentSeatsOfType(x, y int, seatType seatType) int {
	count := 0
	count += btoi(m.Get(x-1, y-1) == seatType)
	count += btoi(m.Get(x+0, y-1) == seatType)
	count += btoi(m.Get(x+1, y-1) == seatType)
	count += btoi(m.Get(x-1, y+0) == seatType)
	count += btoi(m.Get(x+1, y+0) == seatType)
	count += btoi(m.Get(x-1, y+1) == seatType)
	count += btoi(m.Get(x+0, y+1) == seatType)
	count += btoi(m.Get(x+1, y+1) == seatType)
	return count
}

func (m seatMap) HasSeatVisibleInDirection(x, y, dx, dy int, seatType seatType) bool {
	x += dx
	y += dy
	switch m.Get(x, y) {
	case seatType:
		return true
	case seatTypeFloor:
		return m.HasSeatVisibleInDirection(x, y, dx, dy, seatType)
	}
	return false
}

func (m seatMap) Get(x, y int) seatType {
	if x < 0 || y < 0 || x >= m.RowWidth {
		return seatTypeNone
	}
	i := y*m.RowWidth + x
	if i >= len(m.Seats) {
		return seatTypeNone
	}
	return m.Seats[i]
}

func (m seatMap) RearrangeUntilStable(scratch [2]seatMap, rearrange func(_ seatMap, x, y int, _ seatType) seatType) seatMap {
	scratchCounter := 0
	src := &m
	dst := &scratch[scratchCounter]
	for {
		changes := 0
		for i, st := range src.Seats {
			x := i % src.RowWidth
			y := i / src.RowWidth
			newST := rearrange(*src, x, y, st)
			dst.Seats[i] = newST
			if newST != st {
				changes++
			}
		}
		if changes == 0 {
			return *dst
		}
		scratchCounter ^= 1
		dst, src = &scratch[scratchCounter], dst
	}
}

func (m seatMap) CountSeatsOfType(seatType seatType) int {
	count := 0
	for _, st := range m.Seats {
		if st == seatType {
			count++
		}
	}
	return count
}

func (m seatMap) Clone() seatMap {
	seats := make([]seatType, len(m.Seats))
	copy(seats, m.Seats)
	return seatMap{m.RowWidth, seats}
}

type seatType byte

const (
	seatTypeNone seatType = iota
	seatTypeFloor
	seatTypeEmpty
	seatTypeTaken
)

func btoi(b bool) int {
	if b {
		return 1
	}
	return 0
}
