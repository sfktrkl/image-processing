use minifb::{Key, Window, WindowOptions};

pub struct Cell {
    image: Vec<u32>,
    width: usize,
    height: usize,
    x_offset: usize,
    y_offset: usize,
}

pub struct Viewport {
    width: usize,
    height: usize,
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
        let cell_width = image_dimensions.0 as usize;
        let cell_height = image_dimensions.1 as usize;

        let grid_cols = processed.len().min(3);
        let grid_rows = (processed.len() as f32 / grid_cols as f32).ceil() as usize + 1;

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
                let col = index % grid_cols;
                let row = index / grid_cols + 1;

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
            window_width,
            window_height,
            WindowOptions::default(),
        )
        .expect("Unable to create window");

        let window = Viewport {
            width: window_width,
            height: window_height,
            window: popup,
        };

        let buffer = Self::render(cells, window_width, window_height);

        Self { window, buffer }
    }

    pub fn run(&mut self) {
        let window = &mut self.window.window;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            if let Err(error) =
                window.update_with_buffer(&self.buffer, self.window.width, self.window.height)
            {
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
        let image_width = cell.width;
        let image_height = cell.height;

        for y in 0..image_height {
            let target_y = y + cell.y_offset;
            if target_y >= height {
                break;
            }

            for x in 0..image_width {
                let target_x = x + cell.x_offset;
                if target_x >= width {
                    break;
                }

                let pixel_index = y * image_width + x;
                let pixel = cell.image[pixel_index];
                let r = (pixel >> 16 & 0xFF) as u8;
                let g = (pixel >> 8 & 0xFF) as u8;
                let b = (pixel & 0xFF) as u8;

                let target_index = target_y * width + target_x;
                buffer[target_index] = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
            }
        }
    }
}
