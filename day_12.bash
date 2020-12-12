function turn() {
    local direction="$1"; shift
    local hand="$1"; shift
    local times="$1"; shift
    for (( i=0; i<times; i++ )); do
        if [[ "$hand" == R ]]; then
            case "$direction" in
                E)
                    direction="S"
                    ;;
                S)
                    direction="W"
                    ;;
                W)
                    direction="N"
                    ;;
                N)
                    direction="E"
                    ;;
            esac
        else
            case "$direction" in
                E)
                    direction="N"
                    ;;
                S)
                    direction="E"
                    ;;
                W)
                    direction="S"
                    ;;
                N)
                    direction="W"
                    ;;
            esac
        fi
    done
    printf '%s' "$direction"
}

function fumble_instructions() {
    local direction="$1"; shift

    for instruction in $(cat); do
        local step_count="${instruction:1}"
        case "$instruction" in
        R*)
            direction="$(turn "$direction" R $((step_count/90)))"
            ;;
        L*)
            direction="$(turn "$direction" L $((step_count/90)))"
            ;;
        F*)
            echo "${direction}${step_count}"
            ;;
        *)
            echo "$instruction"
            ;;
        esac
    done
}

function adjust_waypoint() {
    local waypoint_x="$1"; shift
    local waypoint_y="$1"; shift

    for instruction in $(cat); do
        local step_count="${instruction:1}"
        case "$instruction" in
        E*)
            waypoint_x=$((waypoint_x+step_count))
            ;;
        S*)
            waypoint_y=$((waypoint_y+step_count))
            ;;
        W*)
            waypoint_x=$((waypoint_x-step_count))
            ;;
        N*)
            waypoint_y=$((waypoint_y-step_count))
            ;;
        R*)
            for (( i=0; i<step_count/90; i++ )); do
                local tmp="$waypoint_x"
                waypoint_x=$((-waypoint_y))
                waypoint_y=$((tmp))
            done
            ;;
        L*)
            for (( i=0; i<step_count/90; i++ )); do
                local tmp="$waypoint_x"
                waypoint_x=$((waypoint_y))
                waypoint_y=$((-tmp))
            done
            ;;
        *)
            echo "INVALID INSTRUCTION $instruction" >&2
            exit 1
        esac
        echo "$waypoint_x $waypoint_y"
    done
}

function navigate() {
    local waypoint_x="$1"; shift
    local waypoint_y="$1"; shift
    local x="$1"; shift
    local y="$1"; shift

    for instruction in $(cat); do
        local step_count="${instruction:1}"
        case "$instruction" in
        F*)
            x=$((x+waypoint_x*step_count))
            y=$((y+waypoint_y*step_count))
            ;;
        *)
            new_waypoint=($(echo "$instruction" | adjust_waypoint "$waypoint_x" "$waypoint_y"))
            waypoint_x="${new_waypoint[0]}"
            waypoint_y="${new_waypoint[1]}"
        esac
        echo "$x $y"
    done
}

function manhattan_distance() {
    local x_from="$1"; shift
    local y_from="$1"; shift
    local x_to="$1"; shift
    local y_to="$1"; shift
    local distance_x=$((x_from-x_to))
    local distance_y=$((y_from-y_to))
    local abs_x="${distance_x#-}"
    local abs_y="${distance_y#-}"
    printf '%s' $((abs_x+abs_y))
}

input="$(cat)"
echo first answer: $(manhattan_distance 0 0 $(echo "$input" | fumble_instructions E | adjust_waypoint E 0 0 | tail -1))
echo second answer: $(manhattan_distance 0 0 $(echo "$input" | navigate 10 -1 0 0 | tail -1))
