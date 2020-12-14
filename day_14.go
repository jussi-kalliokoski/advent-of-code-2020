package main

import (
	"bytes"
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"strconv"
)

func main() {
	instructions, err := parseInstructions(os.Stdin)
	if err != nil {
		panic(err)
	}
	var bm bitmask
	for i := 0; i < bitmaskSize; i++ {
		bm[i] = 1
	}
	bm[35] = 0
	bm[34] = 2
	bm[33] = 1
	bm[32] = 1
	bm[31] = 2
	bm[30] = 0
	fmt.Println("first answer:", applyInstructionsV1(instructions))
	fmt.Println("second answer:", applyInstructionsV2(instructions))
}

func applyInstructionsV2(instructions []instruction) uint64 {
	mem := map[uint64]uint64{}
	addresses := make([]uint64, 0)
	sum := uint64(0)
	for _, instruction := range instructions {
		for _, value := range instruction.values {
			addresses := instruction.bitmask.applyV2(addresses[:0], uint64(value.index))
			for _, address := range addresses {
				sum -= mem[address]
				mem[address] = value.value
				sum += mem[address]
			}
		}
	}
	return sum
}

func applyInstructionsV1(instructions []instruction) uint64 {
	mem := map[int]uint64{}
	sum := uint64(0)
	for _, instruction := range instructions {
		for _, value := range instruction.values {
			sum -= mem[value.index]
			mem[value.index] = instruction.bitmask.applyV1(value.value)
			sum += mem[value.index]
		}
	}
	return sum
}

func parseInstructions(r io.Reader) ([]instruction, error) {
	data, err := ioutil.ReadAll(r)
	if err != nil {
		return nil, err
	}
	instructions := make([]instruction, 0)
	lines := bytes.Split(data, []byte{'\n'})
	k := 0
	for k < len(lines) {
		var instruction instruction
		for i, v := range lines[k][7:] {
			switch v {
			case 'X':
				instruction.bitmask[i] = bitmaskEntryFloating
			case '0':
				instruction.bitmask[i] = bitmaskEntry0
			case '1':
				instruction.bitmask[i] = bitmaskEntry1
			}
		}
		k++
		for k < len(lines) && lines[k][1] == 'e' {
			parts := bytes.Split(lines[k][4:], []byte("] = "))
			index, err := strconv.Atoi(string(parts[0]))
			if err != nil {
				return nil, err
			}
			value, err := strconv.Atoi(string(parts[1]))
			if err != nil {
				return nil, err
			}
			instruction.values = append(instruction.values, instructionValue{int(index), uint64(value)})
			k++
		}
		instructions = append(instructions, instruction)
	}
	return instructions, nil
}

func allocateMem(instructions []instruction) []uint64 {
	maxIndex := 0
	for _, instruction := range instructions {
		for _, value := range instruction.values {
			if value.index > maxIndex {
				maxIndex = value.index
			}
		}
	}
	return make([]uint64, maxIndex+1)
}

type instruction struct {
	bitmask bitmask
	values  []instructionValue
}

type instructionValue struct {
	index int
	value uint64
}

const bitmaskSize = 36

type bitmask [bitmaskSize]bitmaskEntry

func (bm bitmask) applyV1(v uint64) uint64 {
	for i := uint64(0); i < bitmaskSize; i++ {
		if bm[i] == bitmaskEntryFloating {
			continue
		}
		offset := uint64(1 << (bitmaskSize - 1 - i))
		mask := uint64(^offset)
		v &= mask
		if bm[i] == bitmaskEntry1 {
			v += offset
		}
	}
	return v
}

func (bm bitmask) applyV2(dst []uint64, v uint64) []uint64 {
	return bm.applyV2Internal(dst, 0, v)
}

func (bm bitmask) applyV2Internal(dst []uint64, i uint64, v uint64) []uint64 {
	for i < bitmaskSize {
		offset := uint64(1 << (bitmaskSize - 1 - i))
		mask := uint64(^offset)
		switch bm[i] {
		case bitmaskEntryFloating:
			v &= mask
			dst = bm.applyV2Internal(dst, i+1, v)
			v += offset
			dst = bm.applyV2Internal(dst, i+1, v)
			return dst
		case bitmaskEntry1:
			v &= mask
			v += offset
		}
		i++
	}
	dst = append(dst, v)
	return dst
}

type bitmaskEntry uint64

const (
	bitmaskEntryFloating bitmaskEntry = iota
	bitmaskEntry0
	bitmaskEntry1
)
