# Image processing

### Rust Installation

Install [Rust](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify:

```bash
rustc --version
```

### Dependencies

Rust package manager will automatically download the dependencies.

- [`ocl`](https://crates.io/crates/ocl): OpenCL bindings and interfaces.
- [`image`](https://crates.io/crates/image): Imaging library.

### Install OpenCL Runtime for AMD GPUs

- Add the ROCm repository

  ```bash
  sudo apt update
  sudo apt install -y gnupg curl
  curl -sL https://repo.radeon.com/rocm/rocm.gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/rocm.gpg
  echo "deb [arch=amd64 signed-by=/usr/share/keyrings/rocm.gpg] http://repo.radeon.com/rocm/apt/debian/ focal main" | sudo tee /etc/apt/sources.list.d/rocm.list
  ```

- Install the ROCm OpenCL

  ```bash
  sudo apt update
  sudo apt install -y rocm-opencl rocm-opencl-dev
  ```

- Add your user to the video and render groups

  ```bash
  sudo usermod -a -G video $USER
  sudo usermod -a -G render $USER
  ```

- To verify

  ```bash
  sudo apt install clinfo
  clinfo
  ```

- Install OpenCL headers and libraries

  ```bash
  sudo apt install opencl-headers ocl-icd-libopencl1 ocl-icd-opencl-dev
  ```

- Reboot

  ```bash
  sudo reboot
  ```

### Running the Program

- Folder structure

  Ensure that the `input` folder exists in the project directory and contains image files in `.jpg`, `.jpeg`, or `.png` formats.

- Build and run

  ```bash
  cargo run
  ```

- Production build and run

  ```bash
  cargo build --release
  ./target/release/image-processing
  ```
