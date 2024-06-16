use macroquad::prelude::*;

#[macroquad::main("Switcheroo")]
async fn main() {
    let image = load_image("assets/pink_creeper.png").await.unwrap();
    let texture = Texture2D::from_image(&image);

    let mut puzzle = Puzzle::new(400.0, 4, vec2(200.0, 100.0), &texture);

    loop {
        clear_background(WHITE);

        puzzle.update();

        next_frame().await
    }
}

#[derive(Debug)]
struct PuzzlePiece {
    source: Rect,
    dest: Vec2,
    dest_size: Vec2,
    hidden: bool,
}

impl PuzzlePiece {
    fn new(source: Rect, dest: Vec2, dest_size: Vec2) -> Self {
        Self {
            source,
            dest,
            dest_size,
            hidden: false,
        }
    }

    fn draw(&self, texture: &Texture2D) {
        match self.hidden {
            true => {
                draw_rectangle(
                    self.dest.x,
                    self.dest.y,
                    self.dest_size.x,
                    self.dest_size.y,
                    GRAY,
                );
            }
            false => {
                let params = DrawTextureParams {
                    dest_size: Some(self.dest_size),
                    source: Some(self.source),
                    ..DrawTextureParams::default()
                };
                draw_texture_ex(texture, self.dest.x, self.dest.y, WHITE, params);
            }
        }
    }
}

struct PMove {
    from: Vec2,
    to: Vec2,
    from_idx: usize,
    to_idx: usize,
}

struct Puzzle<'a> {
    size: f32,
    length: usize,
    pos: Vec2,
    pieces: Vec<PuzzlePiece>,
    texture: &'a Texture2D,
    hidden_idx: usize,
    moves: Vec<PMove>,
    revving: bool,
}

impl<'a> Puzzle<'a> {
    fn new(size: f32, length: usize, pos: Vec2, texture: &'a Texture2D) -> Self {
        let t_w = texture.width();
        let t_h = texture.height();
        let pieces = Puzzle::gen_pieces(size, length, t_w, t_h, pos);
        // always the last piece for now
        let hidden_idx = pieces.len() - 1;
        Self {
            size,
            length,
            pos,
            pieces,
            texture,
            hidden_idx,
            moves: Vec::new(),
            revving: false,
        }
    }

    fn gen_pieces(size: f32, length: usize, t_w: f32, t_h: f32, pos: Vec2) -> Vec<PuzzlePiece> {
        let cnt = length as f32;

        let source_w = t_w / cnt;
        let source_h = t_h / cnt;

        let piece_x_size = size / cnt;
        let piece_y_size = size / cnt;

        let mut pieces = Vec::new();

        for i in 0..length {
            for j in 0..length {
                let source = Rect {
                    x: source_w * i as f32,
                    y: source_h * j as f32,
                    w: source_w,
                    h: source_h,
                };
                let dest = Vec2 {
                    x: pos.x + (piece_x_size * i as f32),
                    y: pos.y + (piece_y_size * j as f32),
                };
                let dest_size = Vec2 {
                    x: piece_x_size,
                    y: piece_y_size,
                };
                let p = PuzzlePiece::new(source, dest, dest_size);
                pieces.push(p);
            }
        }
        let last = (length * length) - 1;
        pieces[last].hidden = true;
        pieces
    }

    fn update(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.revving = true;
        }
        if self.moves.is_empty() {
            self.revving = false;
        }
        if self.revving {
            let last = self.moves.pop().unwrap();
            self.pieces[self.hidden_idx].dest = last.to;
            self.pieces[last.from_idx].dest = last.from;
            self.hidden_idx = last.to_idx;
        };

        if is_mouse_button_pressed(MouseButton::Left) {
            let (pos_x, pos_y) = mouse_position();
            // click is in bounds
            if self.mouse_is_in_bound(pos_x, pos_y) && !self.mouse_on_hidden(pos_x, pos_y) {
                let found = self.pieces.iter().enumerate().find(|(_, p)| {
                    if p.hidden {
                        return false;
                    };
                    if p.dest.x < pos_x
                        && p.dest.x + p.dest_size.x > pos_x
                        && p.dest.y < pos_y
                        && p.dest.y + p.dest_size.y > pos_y
                    {
                        return true;
                    };
                    false
                });

                if found.is_none() {
                    return;
                }

                let (piece_idx, _) = found.unwrap();

                // only want to swap if we are left, right, up or down
                // left
                let hidden = &self.pieces[self.hidden_idx];
                let curr = &self.pieces[piece_idx];

                if (hidden.dest.y == curr.dest.y && curr.dest.x < hidden.dest.x)
                    || (hidden.dest.y == curr.dest.y && curr.dest.x > hidden.dest.x)
                    || (hidden.dest.x == curr.dest.x && curr.dest.y < hidden.dest.y)
                    || (hidden.dest.x == curr.dest.x && curr.dest.y > hidden.dest.y)
                {
                    self.moves.push(PMove {
                        from: curr.dest,
                        to: hidden.dest,
                        from_idx: piece_idx,
                        to_idx: self.hidden_idx,
                    });
                    let temp_dest: Vec2 = self.pieces[piece_idx].dest;
                    self.pieces[piece_idx].dest = self.pieces[self.hidden_idx].dest;
                    self.pieces[self.hidden_idx].dest = temp_dest;
                }
            }
        };
        self.draw();
    }

    fn mouse_is_in_bound(&self, pos_x: f32, pos_y: f32) -> bool {
        if pos_x > self.pos.x
            && pos_x < self.pos.x + self.size
            && pos_y > self.pos.y
            && pos_y < self.pos.y + self.size
        {
            return true;
        };
        false
    }

    fn mouse_on_hidden(&self, pos_x: f32, pos_y: f32) -> bool {
        let hidden = &self.pieces[self.hidden_idx];
        if hidden.dest.x < pos_x
            && hidden.dest.x + hidden.dest_size.x > pos_x
            && hidden.dest.y < pos_y
            && hidden.dest.y + hidden.dest_size.y > pos_y
        {
            return true;
        };
        false
    }

    fn draw(&self) {
        self.draw_pieces();
        self.draw_borders();
    }

    fn draw_pieces(&self) {
        for piece in &self.pieces {
            piece.draw(self.texture);
        }
    }

    fn draw_borders(&self) {
        let thickness = 10.0;
        draw_rectangle_lines(
            self.pos.x, self.pos.y, self.size, self.size, thickness, GRAY,
        );

        let thickness = 5.0;
        let line_count = self.length;
        let gap = self.size / line_count as f32;

        // horizontal
        for i in 1..=line_count {
            let x1 = self.pos.x;
            let x2 = self.pos.x + self.size;
            let y1 = self.pos.y + (gap * i as f32);
            draw_line(x1, y1, x2, y1, thickness, GRAY);
        }

        // vertical
        for i in 1..=line_count {
            let y1 = self.pos.y;
            let y2 = self.pos.y + self.size;
            let x1 = self.pos.x + (gap * i as f32);
            draw_line(x1, y1, x1, y2, thickness, GRAY);
        }
    }
}
