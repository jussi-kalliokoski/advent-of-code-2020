import fileinput

seat_floor = "."
seat_empty = "L"
seat_taken = "#"

def add_seat_count(counts, seat_type):
    floor_count, empty_count, taken_count = counts
    if seat_type == seat_floor:
        return floor_count + 1, empty_count, taken_count
    if seat_type == seat_empty:
        return floor_count, empty_count + 1, taken_count
    if seat_type == seat_taken:
        return floor_count, empty_count, taken_count + 1

def add_seat(counts, seating, x, y):
    floor_count, empty_count, taken_count = counts
    if y >= 0 and y < len(seating) and x >= 0 and x < len(seating[y]):
        return add_seat_count((floor_count, empty_count, taken_count), seating[y][x])
    return counts

def add_seat_in_direction(counts, seating, x, y, xd, yd):
    floor_count, empty_count, taken_count = counts
    x, y = x + xd, y + yd
    if y >= 0 and y < len(seating) and x >= 0 and x < len(seating[y]):
        if seating[y][x] == seat_floor:
            return add_seat_in_direction(counts, seating, x, y, xd, yd)
        return add_seat_count((floor_count, empty_count, taken_count), seating[y][x])
    return counts

def directions():
    for yd in (-1, +0, +1):
        for xd in (-1, +0, +1):
            if xd != 0 or yd != 0:
                yield xd, yd

def adjacent_seat_types(seating, x, y):
    floor_count, empty_count, taken_count = 0, 0, 0
    for xd, yd in directions():
        floor_count, empty_count, taken_count = add_seat((floor_count, empty_count, taken_count), seating, x + xd, y + yd)
    return floor_count, empty_count, taken_count

def seat_types_in_all_directions(seating, x, y):
    floor_count, empty_count, taken_count = 0, 0, 0
    for xd, yd in directions():
        floor_count, empty_count, taken_count = add_seat_in_direction((floor_count, empty_count, taken_count), seating, x, y, xd, yd)
    return floor_count, empty_count, taken_count

def count_seat_types(seating):
    floor_count, empty_count, taken_count = 0, 0, 0
    for row in seating:
        for seat_type in row:
            floor_count, empty_count, taken_count = add_seat_count((floor_count, empty_count, taken_count), seat_type)
    return floor_count, empty_count, taken_count

def copy_seating(seating):
    return [[seat_type for x, seat_type in enumerate(row)] for y, row in enumerate(seating)]

def map_seating(seating, transform):
    return [[transform(seating, x, y, seat_type) for x, seat_type in enumerate(row)] for y, row in enumerate(seating)]

def count_remaining_seats_after_stabilization(seating, rearrange):
    floor_count, empty_count, taken_count = count_seat_types(seating)
    while True:
        old_floor_count, old_empty_count, old_taken_count = floor_count, empty_count, taken_count
        seating = map_seating(seating, rearrange)
        floor_count, empty_count, taken_count = count_seat_types(seating)
        if old_floor_count == floor_count and old_empty_count == empty_count and old_taken_count == taken_count:
            break
    return floor_count, empty_count, taken_count

def rearrange_seating(seating, x, y, seat_type):
    if seat_type == seat_empty:
        _, _, adjacent_taken_count = seat_types_in_all_directions(seating, x, y)
        if adjacent_taken_count == 0:
            return seat_taken
    if seat_type == seat_taken:
        _, _, adjacent_taken_count = seat_types_in_all_directions(seating, x, y)
        if adjacent_taken_count >= 5:
            return seat_empty
    return seat_type

def rearrange_seating_naive(seating, x, y, seat_type):
    if seat_type == seat_empty:
        _, _, adjacent_taken_count = adjacent_seat_types(seating, x, y)
        if adjacent_taken_count == 0:
            return seat_taken
    if seat_type == seat_taken:
        _, _, adjacent_taken_count = adjacent_seat_types(seating, x, y)
        if adjacent_taken_count >= 4:
            return seat_empty
    return seat_type

seating = [[c for c in l.rstrip()] for l in fileinput.input()]
_, _, first_answer = count_remaining_seats_after_stabilization(seating, rearrange_seating_naive)
print("first answer:", first_answer)
_, _, second_answer = count_remaining_seats_after_stabilization(seating, rearrange_seating)
print("second answer:", second_answer)
