fn count_checkerboard(width: u128, height: u128, resolution: u128) -> u128 {
    let cols = width / resolution;
    let rows = height / resolution;
    let rem_x = width % resolution;
    let rem_y = height % resolution;

    let full_blocks = rows * cols;
    let black_full_blocks = full_blocks / 2;
    let area_full = black_full_blocks * resolution * resolution;

    let black_right_blocks = rows / 2 + (rows % 2) * (cols % 2);
    let area_right = black_right_blocks * rem_x * resolution;

    let black_bottom_blocks = cols / 2 + (cols % 2) * (rows % 2);
    let area_bottom = black_bottom_blocks * rem_y * resolution;

    let corner_is_black = (cols + rows) % 2;
    let area_corner = corner_is_black * rem_x * rem_y;

    area_full + area_right + area_bottom + area_corner
}

pub fn count_checkerboard_alternative(width: u128, height: u128, resolution: u128) -> u128 {
    let mut black_pixels = 0;

    let mut y = 0;
    let mut row_idx = 0;

    let cols = width / resolution;
    let rem_x = width % resolution;

    while y < height {
        let current_h = if y + resolution > height { height - y } else { resolution };
        let mut black_full_blocks = cols / 2;
        let row_starts_black = row_idx % 2 != 0;

        if cols % 2 != 0 && row_starts_black {
            black_full_blocks += 1;
        }

        black_pixels += black_full_blocks * resolution * current_h;

        let remainder_is_black = (row_idx + cols) % 2 != 0;
        if remainder_is_black {
            black_pixels += rem_x * current_h;
        }

        y += resolution;
        row_idx += 1;
    }

    black_pixels
}

#[cfg(test)]
mod tests {
    use super::count_checkerboard;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(w: u128, h: u128, r: u128, expected: u128) {
        assert_eq!(
            count_checkerboard(w, h, r),
            expected,
            "{ERR_MSG} with width = {w}, height = {h}, resolution = {r}"
        )
    }

    #[test]
    fn small_examples() {
        dotest(11, 6, 1, 33);
        dotest(11, 6, 2, 32);
        dotest(11, 6, 5, 31);
        dotest(9, 5, 2, 22);
        dotest(9, 5, 4, 21);
        dotest(9, 5, 8, 5);
    }
    #[test]
    fn larger_examples() {
        dotest(123456, 7654321, 333, 472485924597);
        dotest(10_u128.pow(10), 10, 20, 5 * 10_u128.pow(10));
        dotest(10_u128.pow(10), 11, 21, 54999999978);
        dotest(8_u128.pow(5), 7_u128.pow(9), 124, 661153496464);
    }
    #[test]
    fn some_edge_cases() {
        dotest(0, 123, 1, 0);
        dotest(445, 998, 101010, 0);
        dotest(0, 0, 1, 0);
        dotest(8, 9, 7, 21);
    }
}
