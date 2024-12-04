pub struct ImageConverter;

impl ImageConverter {
    pub fn convert_rgb_to_grayscale(pixels: &[u32]) -> Vec<f32> {
        pixels
            .chunks(1)
            .map(|pixel| {
                let r = ((pixel[0] >> 16) & 0xFF) as f32 / 255.0;
                let g = ((pixel[0] >> 8) & 0xFF) as f32 / 255.0;
                let b = (pixel[0] & 0xFF) as f32 / 255.0;

                0.2989 * r + 0.5870 * g + 0.1140 * b
            })
            .collect()
    }

    pub fn convert_grayscale_to_rgb(pixels: &[f32]) -> Vec<u32> {
        pixels
            .iter()
            .map(|&value| {
                let intensity = (value * 255.0).clamp(0.0, 255.0) as u8;

                (intensity as u32) << 16 | (intensity as u32) << 8 | (intensity as u32)
            })
            .collect()
    }

    pub fn decompose_rgb(input: &[u32]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let mut r_channel = Vec::with_capacity(input.len());
        let mut g_channel = Vec::with_capacity(input.len());
        let mut b_channel = Vec::with_capacity(input.len());

        for &pixel in input {
            let r = ((pixel >> 16) & 0xFF) as f32 / 255.0;
            let g = ((pixel >> 8) & 0xFF) as f32 / 255.0;
            let b = (pixel & 0xFF) as f32 / 255.0;
            r_channel.push(r);
            g_channel.push(g);
            b_channel.push(b);
        }

        (r_channel, g_channel, b_channel)
    }

    pub fn recompose_rgb(r_channel: &[f32], g_channel: &[f32], b_channel: &[f32]) -> Vec<u32> {
        let mut output = Vec::with_capacity(r_channel.len());

        for i in 0..r_channel.len() {
            let r = (r_channel[i].min(1.0).max(0.0) * 255.0) as u32;
            let g = (g_channel[i].min(1.0).max(0.0) * 255.0) as u32;
            let b = (b_channel[i].min(1.0).max(0.0) * 255.0) as u32;
            let a = 255; // Fully opaque

            output.push((a << 24) | (r << 16) | (g << 8) | b);
        }

        output
    }

    pub fn recompose_rgb_with_original(
        r_channel: &[f32],
        g_channel: &[f32],
        b_channel: &[f32],
        original: &[u32],
    ) -> Vec<u32> {
        let mut output = Vec::with_capacity(r_channel.len());

        for i in 0..r_channel.len() {
            let orig_r = ((original[i] >> 16) & 0xFF) as f32 / 255.0;
            let orig_g = ((original[i] >> 8) & 0xFF) as f32 / 255.0;
            let orig_b = (original[i] & 0xFF) as f32 / 255.0;

            let r = ((r_channel[i] + orig_r).min(1.0).max(0.0) * 255.0) as u32;
            let g = ((g_channel[i] + orig_g).min(1.0).max(0.0) * 255.0) as u32;
            let b = ((b_channel[i] + orig_b).min(1.0).max(0.0) * 255.0) as u32;
            let a = 255; // Fully opaque

            output.push((a << 24) | (r << 16) | (g << 8) | b);
        }

        output
    }
}
