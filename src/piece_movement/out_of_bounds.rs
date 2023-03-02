pub fn out_of_bounds(rank: i8, file: i8) -> bool {
    !(rank >= 0 && rank <= 7 && file >= 0 && file <= 7)
}    