use console::Term;

pub fn draw_pixels(pixels: &[bool], w: usize, h: usize) {
    let term = Term::stdout();
    let frame = pixels
        .chunks(w)
        .map(|row| {
            row.iter().map(|on| if *on { "*" } else { " " }).collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");
    term.write_str(&frame);
    term.clear_last_lines(h);
}
