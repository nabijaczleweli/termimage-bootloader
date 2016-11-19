pub fn fname_to_8_3(fname: &str) -> String {
    fname.trim()
        .to_uppercase()
        .chars()
        .filter(|&c| {
            (c as u32 >= 128 && c as u32 <= 255) ||
            ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2',
             '3', '4', '5', '6', '7', '8', '9', '!', '#', '$', '%', '&', '\'', '(', ')', '-', '@', '^', '_', '`', '{', '}', '~']
                .contains(&c)
        })
        .take(8)
        .collect()
}
