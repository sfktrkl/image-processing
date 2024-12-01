use minifb::{Key, Window, WindowOptions};

pub struct Cell {
    image: Vec<u32>,
    width: u32,
    height: u32,
    x_offset: u32,
    y_offset: u32,
}

pub struct Viewport {
    width: u32,
    height: u32,
    window: Window,
}

pub struct ImageViewer {
    window: Viewport,
    buffer: Vec<u32>,
}

impl ImageViewer {
    pub fn new(
        original: Vec<u32>,
        processed: Vec<Vec<u32>>,
        image_dimensions: (u32, u32),
    ) -> ImageViewer {
        let cell_width = image_dimensions.0;
        let cell_height = image_dimensions.1;

        let grid_cols = processed.len().min(3) as u32;
        let grid_rows = (processed.len() as f32 / grid_cols as f32).ceil() as u32 + 1;

        let window_width = cell_width * grid_cols;
        let window_height = cell_height * grid_rows;

        let original_image = Cell {
            image: original,
            width: cell_width,
            height: cell_height,
            x_offset: (window_width - cell_width) / 2,
            y_offset: 0,
        };

        let processed_images: Vec<Cell> = processed
            .into_iter()
            .enumerate()
            .map(|(index, processed_image)| {
                let col = index as u32 % grid_cols;
                let row = index as u32 / grid_cols + 1;

                Cell {
                    image: processed_image,
                    width: cell_width,
                    height: cell_height,
                    x_offset: col * cell_width,
                    y_offset: row * cell_height,
                }
            })
            .collect();

        let mut cells = vec![original_image];
        cells.extend(processed_images);

        let popup = Window::new(
            "Image Viewer",
            window_width as usize,
            window_height as usize,
            WindowOptions::default(),
        )
        .expect("Unable to create window");

        let window = Viewport {
            width: window_width,
            height: window_height,
            window: popup,
        };

        let buffer = Self::render(cells, window_width as usize, window_height as usize);

        Self { window, buffer }
    }

    pub fn run(&mut self) {
        let window = &mut self.window.window;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            if let Err(error) = window.update_with_buffer(
                &self.buffer,
                self.window.width as usize,
                self.window.height as usize,
            ) {
                eprintln!("Error updating buffer: {:?}", error);
            }
        }
    }

    fn render(cells: Vec<Cell>, width: usize, height: usize) -> Vec<u32> {
        let mut buffer = vec![0; width * height];

        for cell in cells {
            Self::render_cell(&cell, &mut buffer, width, height);
        }

        buffer
    }

    fn render_cell(cell: &Cell, buffer: &mut [u32], width: usize, height: usize) {
        let image_width = cell.width as usize;
        let image_height = cell.height as usize;

        for y in 0..image_height {
            let target_y = y + cell.y_offset as usize;
            if target_y >= height as usize {
                break;
            }

            for x in 0..image_width {
                let target_x = x + cell.x_offset as usize;
                if target_x >= width as usize {
                    break;
                }

                let pixel_index = y * image_width + x;
                let pixel = cell.image[pixel_index];
                let r = (pixel >> 16 & 0xFF) as u8;
                let g = (pixel >> 8 & 0xFF) as u8;
                let b = (pixel & 0xFF) as u8;

                let target_index = target_y * width as usize + target_x;
                buffer[target_index] = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
            }
        }
    }
}
