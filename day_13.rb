#!/usr/bin/env ruby

first_timestamp_raw, schedules_raw = STDIN.read.split("\n")
first_timestamp = Integer(first_timestamp_raw)
schedules = schedules_raw.split(",").map { |str| str == "x" ? 1 : Integer(str, 10) }

next_bus, next_bus_departs = schedules
  .filter { |schedule| schedule != 1 }
  .map { |schedule| [schedule, schedule - (first_timestamp % schedule)] }
  .sort_by { |departure| departure[1] }
  .first
puts "first answer: " + (next_bus * next_bus_departs).to_s

puts "second answer: " + schedules
  .each_with_index
  .map { |schedule, index| [schedule, index] }
  .to_a
  .reduce([0, schedules[0]]) { |acc, schedule_with_index|
    prev_common, prev_gap = acc
    repeats_every, repeats_offset = schedule_with_index
    common, gap = 0, 0
    multiplier = 0
    while true
      time = prev_common + prev_gap * multiplier
      if (time + repeats_offset) % repeats_every == 0
        if common == 0
          common = time
        else
          gap = time - common
          break
        end
      end
      multiplier += 1
    end
    [common, gap]
  }
  .first
  .to_s
