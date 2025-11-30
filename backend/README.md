
## 📄➡️🖼️ Pdf2Image Web
[![Rust](https://img.shields.io/badge/Language-Rust-000000?style=flat&logo=rust)](https://www.rust-lang.org/)
[![Salvo](https://img.shields.io/badge/Framework-Salvo-orange?style=flat&logo=rust)](https://salvo.rs/)
[![Docker](https://img.shields.io/badge/Docker-ready-blue?style=flat&logo=docker&logoColor=white)](https://www.docker.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast and lightweight web application that converts PDF pages into high-quality images, offering seamless processing for users and effortless integration into automation or backend workflows. Easily preview results and download all converted images as a ZIP archive. ⚡

---

🚀 Features

- 🖼️ Convert each PDF page into high-quality PNG or JPG

- 🔍 Fine-tune output with custom DPI

- 📁 Support for custom output directories

- 🧰 Simple installation via Cargo

- ⚡ Powered by Rust + Salvo for exceptional performance

- 🐳 Ready for deployment with Docker

---

### 🐳 Build & Run with Docker

Deploy Pdf2Image Web effortlessly using Docker. Just build the image and run the container — no manual setup required. 🚀

📦 Build the Docker Image

```bash
docker build -t pdf2images_app .\
```

▶️ Run the Container

```bash
docker run -p 5800:5800 --name pdf2images_app pdf2images_app:latest
```

Your application will now be available at [http://localhost:5800](http://localhost:5800)🎉

### 📥 Installing `libpdfium.so`

To use pdfium-render, you need the `libpdfium.so` binary for your system.

1. Clone the official pdfium-binaries repository:

```bash
git clone https://github.com/bblanchon/pdfium-binaries
cd pdfium-binaries
```

2. Build the library according to your OS and architecture.
For example, on **Linux x64**: 

```bash
./build.sh linux x64
```

3. After the build completes, the compiled `libpdfium.so` will be available inside the build output directory.

Copy it into your project’s expected library path (e.g., `./pdfium/` or `/usr/lib/`).

---

### 🚀 Launch Using cURL

You can easily interact with the web application directly from your command-line using curl.
Just send your PDF file and download the resulting ZIP containing all converted images. 📄➡️🖼️📦

You can also pass query parameters such as image format (png, jpg) and DPI (e.g., 150, 300) to control the output quality. 🎛️🖼️

🧪 Example Command

```bash
curl -X POST "http://127.0.0.1:5800/convert?format=png&dpi=150" \
  -F "pdf=@document.pdf" \
  -o output.zip
```