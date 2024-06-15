use macroquad::prelude::*;

#[derive(Debug)]
struct PuzzlePiece {
    source: Rect,
    dest: Vec2,
    dest_size: Vec2,
}

impl PuzzlePiece {
    fn new(source: Rect, dest: Vec2, dest_size: Vec2) -> Self {
        Self {
            source,
            dest,
            dest_size,
        }
    }

    fn draw(&self, texture: &Texture2D) {
        let params = DrawTextureParams {
            dest_size: Some(self.dest_size),
            source: Some(self.source),
            ..DrawTextureParams::default()
        };
        draw_texture_ex(texture, self.dest.x, self.dest.y, WHITE, params);
    }
}

struct Puzzle<'a> {
    size: f32,
    length: usize,
    pos: Vec2,
    pieces: Vec<PuzzlePiece>,
    texture: &'a Texture2D,
}

impl<'a> Puzzle<'a> {
    fn new(size: f32, length: usize, pos: Vec2, texture: &'a Texture2D) -> Self {
        let t_w = texture.width();
        let t_h = texture.height();
        let pieces = Puzzle::gen_pieces(size, length, t_w, t_h, pos);
        Self {
            size,
            length,
            pos,
            pieces,
            texture,
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
        pieces
    }

    fn draw(&self) {
        self.draw_pieces();
        self.draw_borders();
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

    fn draw_pieces(&self) {
        for piece in &self.pieces {
            piece.draw(self.texture);
        }
    }
}

#[macroquad::main("Switcheroo")]
async fn main() {
    let image = load_image("assets/pink_creeper.png").await.unwrap();
    let texture = Texture2D::from_image(&image);

    let puzzle = Puzzle::new(400.0, 4, vec2(200.0, 100.0), &texture);

    loop {
        clear_background(WHITE);

        puzzle.draw();

        next_frame().await
    }
}
