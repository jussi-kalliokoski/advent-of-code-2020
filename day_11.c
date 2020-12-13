#include <stdio.h>
#include <stdlib.h>

typedef enum {
    seat_none,
    seat_floor,
    seat_empty,
    seat_taken
} seat_t;

typedef struct {
    int width;
    int height;
    seat_t* seats;
} seat_map_t;

seat_map_t* seat_map_parse(FILE* fd) {
    const size_t buf_size = 0xff;
    char buf[buf_size];
    int width = 0;
    int height = 1;
    size_t seats_size = 0xff;
    size_t seats_i = 0;
    seat_t* seats = (seat_t*)(malloc(sizeof(seat_t) * seats_size));
    while (1) {
        size_t bytes_read = fread(buf, sizeof(char), buf_size, fd);
        for (int i = 0; i < bytes_read; i++) {
            width++;
            switch (buf[i]) {
                case '\n':
                    width = 0;
                    height++;
                    break;
                case '.':
                    seats[seats_i++] = seat_floor;
                    break;
                case 'L':
                    seats[seats_i++] = seat_empty;
                    break;
                case '#':
                    seats[seats_i++] = seat_taken;
                    break;
            }
            if (seats_size == seats_i) {
                seats_size <<= 1;
                seats = (seat_t*)(realloc(seats, sizeof(seats_size) * seats_size));
            }
        }
        if (bytes_read < buf_size) {
            break;
        }
    }
    seat_map_t* m = (seat_map_t*)(malloc(sizeof(seat_map_t)));
    m->width = width;
    m->height = height;
    m->seats = seats;
    return m;
}

void seat_map_destroy(seat_map_t* self) {
    if ((void*)(self->seats) != (void*)(self + sizeof(seat_map_t))) {
        // for clones the data is inline so this can be skipped
        free(self->seats);
    }
    free(self);
}

seat_map_t* seat_map_clone_dirty(seat_map_t* self) {
    int l = self->width * self->height;
    seat_map_t* m = (seat_map_t*)(malloc(sizeof(seat_map_t) + sizeof(seat_t) * l));
    m->width = self->width;
    m->height = self->height;
    m->seats = (seat_t*)(m + sizeof(seat_map_t));
    return m;
}

seat_t seat_map_get_seat(seat_map_t* self, int x, int y) {
    if (x < 0 || y < 0 || x >= self->width || y >= self->height) {
        return seat_none;
    }
    return self->seats[y * self->width + x];
}

int seat_map_has_seat_visible_in_direction(seat_map_t* self, int x, int y, int dx, int dy, seat_t seat_type) {
    while (1) {
        x += dx;
        y += dy;
        seat_t found = seat_map_get_seat(self, x, y);
        if (found == seat_type) {
            return 1;
        }
        if (found != seat_floor) {
            return 0;
        }
    }
}

size_t seat_map_count_visible_seats_of_type(seat_map_t* self, int x, int y, seat_t seat_type) {
    size_t count = 0;
	count += seat_map_has_seat_visible_in_direction(self, x, y, -1, -1, seat_type);
	count += seat_map_has_seat_visible_in_direction(self, x, y, +0, -1, seat_type);
	count += seat_map_has_seat_visible_in_direction(self, x, y, +1, -1, seat_type);
	count += seat_map_has_seat_visible_in_direction(self, x, y, -1, +0, seat_type);
	count += seat_map_has_seat_visible_in_direction(self, x, y, +1, +0, seat_type);
	count += seat_map_has_seat_visible_in_direction(self, x, y, -1, +1, seat_type);
	count += seat_map_has_seat_visible_in_direction(self, x, y, +0, +1, seat_type);
	count += seat_map_has_seat_visible_in_direction(self, x, y, +1, +1, seat_type);
	return count;
}

size_t seat_map_count_adjacent_seats_of_type(seat_map_t* self, int x, int y, seat_t seat_type) {
    size_t count = 0;
	count += seat_map_get_seat(self, x - 1, y - 1) == seat_type;
	count += seat_map_get_seat(self, x + 0, y - 1) == seat_type;
	count += seat_map_get_seat(self, x + 1, y - 1) == seat_type;
	count += seat_map_get_seat(self, x - 1, y + 0) == seat_type;
	count += seat_map_get_seat(self, x + 1, y + 0) == seat_type;
	count += seat_map_get_seat(self, x - 1, y + 1) == seat_type;
	count += seat_map_get_seat(self, x + 0, y + 1) == seat_type;
	count += seat_map_get_seat(self, x + 1, y + 1) == seat_type;
	return count;
}

int seat_map_count_seats_of_type(seat_map_t* self, seat_t seat_type) {
    int count = 0;
    int l = self->width * self->height;
    for (int i = 0; i < l; i++) {
        if (self->seats[i] == seat_type) {
            count++;
        }
    }
    return count;
}

int seat_map_rearrange(seat_map_t* self, seat_map_t* src, seat_t (*rearrange)(seat_map_t*, int, int, seat_t)) {
    int changes = 0;
    int l = src->width * src->height;
    for (int i = 0; i < l; i++) {
        int x = i % src->width;
        int y = i / src->width;
        seat_t old = src->seats[i];
        seat_t new = rearrange(src, x, y, old);
        self->seats[i] = new;
        changes += old != new;
    }
    return changes;
}

seat_map_t* seat_map_rearrange_until_stable(seat_map_t* self, seat_map_t* scratch[2], seat_t (*rearrange)(seat_map_t*, int, int, seat_t)) {
    int scratch_i = 0;
    seat_map_t* src = self;
    seat_map_t* dst = scratch[scratch_i];
    while (1) {
        if (!seat_map_rearrange(dst, src, rearrange)) {
            return dst;
        }
        scratch_i ^= 1;
        src = dst;
        dst = scratch[scratch_i];
    }
}

seat_t rearrange_1(seat_map_t* m, int x, int y, seat_t seat_type) {
    switch (seat_type) {
        case seat_empty:
            if (seat_map_count_adjacent_seats_of_type(m, x, y, seat_taken) == 0) {
                return seat_taken;
            }
            break;
        case seat_taken:
            if (seat_map_count_adjacent_seats_of_type(m, x, y, seat_taken) >= 4) {
                return seat_empty;
            }
            break;
        default:
            break;
    }
    return seat_type;
}

seat_t rearrange_2(seat_map_t* m, int x, int y, seat_t seat_type) {
    switch (seat_type) {
        case seat_empty:
            if (seat_map_count_visible_seats_of_type(m, x, y, seat_taken) == 0) {
                return seat_taken;
            }
            break;
        case seat_taken:
            if (seat_map_count_visible_seats_of_type(m, x, y, seat_taken) >= 5) {
                return seat_empty;
            }
            break;
        default:
            break;
    }
    return seat_type;
}

int main() {
    seat_map_t* m = seat_map_parse(stdin);
    seat_map_t* scratch[2] = { seat_map_clone_dirty(m), seat_map_clone_dirty(m) };
    printf("first answer: %d\n", seat_map_count_seats_of_type(seat_map_rearrange_until_stable(m, scratch, rearrange_1), seat_taken));
    printf("second answer: %d\n", seat_map_count_seats_of_type(seat_map_rearrange_until_stable(m, scratch, rearrange_2), seat_taken));
    seat_map_destroy(scratch[0]);
    seat_map_destroy(scratch[1]);
    seat_map_destroy(m);
    return 0;
}
