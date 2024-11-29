use minifb::Key;

pub struct Cell {
    width: u32,
    height: u32,
    x_offset: u32,
    y_offset: u32,
    image: Vec<u32>,
}

pub struct Window {
    width: u32,
    height: u32,
    window: minifb::Window,
}

pub struct ImageViewer {
    window: Window,
    cells: Vec<Cell>,
    buffer: Option<Vec<u32>>,
}

impl ImageViewer {
    pub fn new(
        original: Vec<u32>,
        processed: Vec<Vec<u32>>,
        image_dimensions: (u32, u32),
    ) -> ImageViewer {
        let cell_width = image_dimensions.0;
        let cell_height = image_dimensions.1;

        let original_image = Cell {
            x_offset: 0,
            y_offset: 0,
            width: cell_width,
            height: cell_height,
            image: original,
        };

        let processed_images: Vec<Cell> = processed
            .into_iter()
            .map(|processed_image| Cell {
                x_offset: 0,
                y_offset: 0,
                width: cell_width,
                height: cell_height,
                image: processed_image,
            })
            .collect();

        let grid_cols = if processed_images.len() <= 3 {
            processed_images.len() as u32
        } else {
            3
        };
        let grid_rows = (processed_images.len() as f32 / grid_cols as f32).ceil() as u32 + 1;

        let mut cells = vec![original_image];
        cells.extend(processed_images);

        let window_width = cell_width * grid_cols;
        let window_height = cell_height * grid_rows;

        let popup = minifb::Window::new(
            "Image Viewer",
            window_width as usize,
            window_height as usize,
            minifb::WindowOptions::default(),
        )
        .expect("Unable to create window");

        let window = Window {
            width: window_width,
            height: window_height,
            window: popup,
        };

        if let Some(first_cell) = cells.first_mut() {
            first_cell.x_offset = ((window_width - first_cell.width) / 2) as u32;
        }

        for (index, cell) in cells.iter_mut().skip(1).enumerate() {
            let col = index % grid_cols as usize;
            let row = index / grid_cols as usize + 1;
            cell.x_offset = (col * cell.width as usize) as u32;
            cell.y_offset = row as u32 * cell.height;
        }

        Self {
            window,
            cells,
            buffer: None,
        }
    }

    pub fn run(&mut self) {
        if self.buffer.is_none() {
            self.buffer = Some(self.render());
        }

        let window = &mut self.window.window;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            let width = self.window.width as usize;
            let height = self.window.height as usize;
            window
                .update_with_buffer(&self.buffer.as_ref().unwrap(), width, height)
                .unwrap();
        }
    }

    fn render(&self) -> Vec<u32> {
        let mut buffer = vec![0; (self.window.width * self.window.height) as usize];

        for cell in &self.cells {
            self.render_cell(&cell, &mut buffer);
        }

        buffer
    }

    fn render_cell(&self, cell: &Cell, buffer: &mut [u32]) {
        let image_width = cell.width as usize;
        let image_height = cell.height as usize;

        for y in 0..image_height {
            let target_y = y + cell.y_offset as usize;
            if target_y >= self.window.height as usize {
                break;
            }

            for x in 0..image_width {
                let target_x = x + cell.x_offset as usize;
                if target_x >= self.window.width as usize {
                    break;
                }

                let pixel_index = y * image_width + x;
                let pixel = cell.image[pixel_index];
                let r = (pixel >> 16 & 0xFF) as u8;
                let g = (pixel >> 8 & 0xFF) as u8;
                let b = (pixel & 0xFF) as u8;

                let target_index = target_y * self.window.width as usize + target_x;
                buffer[target_index] = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
            }
        }
    }
}
