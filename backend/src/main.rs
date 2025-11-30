use salvo::{
    cors::{AllowOrigin, Cors}, http::Method, oapi::extract::QueryParam, prelude::*
};
use pdfium_render::prelude::*;
use std::io::{Cursor, Write};
use uuid::Uuid;
use zip::write::FileOptions;

#[endpoint]
async fn convert_pdf(dpi: QueryParam<u16, false>, format: QueryParam<String, false>, password: QueryParam<String, false>, req: &mut Request, res: &mut Response,) {
    // --- Extract file first (no conflict with req.query) because we use QueryParam
    let file = match req.file("pdf").await {
        Some(f) => f,
        None => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render("PDF file is required");
            return;
        }
    };

    // --- Load PDF using Pdfium
    // Load Pdfium (uses the bundled binary if available)

    let pdfium = Pdfium::default();

    let document = match pdfium.load_pdf_from_file(file.path(), password.as_deref()) {
        Ok(doc) => doc,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Failed to load PDF: {}", e));
            return;
        }
    };

    // --- We create ZIP buffer AFTER pdfium load
    // let mut zip_buffer = Vec::new();
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let mut zip_writer = zip::ZipWriter::new(&mut cursor);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);

    let dpi: u16 = dpi.unwrap_or(150);
    let format: String = format.clone().unwrap_or("png".to_string());

    println!("✅ PDF started converted to images ");

    for (index, page) in document.pages().iter().enumerate() {
        let render = match page.render_with_config(
            &PdfRenderConfig::new()
                .set_target_width((dpi as f32 * 8.5) as i32)
                .set_target_height((dpi as f32 * 11.0) as i32)
                .render_form_data(true),
        ) {
            Ok(r) => r,
            Err(e) => {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(format!("Render error: {}", e));
                return;
            }
        };

        let image = render.as_image();
        let filename = format!("page_{}.{}", index + 1, format);

        zip_writer.start_file(filename, options).unwrap();

        let mut img_cursor = Cursor::new(Vec::<u8>::new());

        match format.as_str() {
            "png" => image.write_to(&mut img_cursor, image::ImageFormat::Png).unwrap(),
            "jpg" | "jpeg" => image.write_to(&mut img_cursor, image::ImageFormat::Jpeg).unwrap(),
            _ => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render("Unsupported format");
                return;
            }
        }

        let buffer = img_cursor.into_inner();
        zip_writer.write_all(&buffer).unwrap();
    }

    zip_writer.finish().unwrap();

    // Borrow ended
    drop(zip_writer);
    
    let zip_buffer = cursor.into_inner();

    // --- Prepare response
    let filename = format!("converted-{}.zip", Uuid::new_v4());
    res.headers_mut()
        .insert("Content-Type", "application/zip".parse().unwrap());
    res.headers_mut().insert(
        "Content-Disposition",
        format!("attachment; filename=\"{}\"", filename)
            .parse()
            .unwrap(),
    );

    let _ = res.write_body(zip_buffer);
    println!("✅ PDF successfully converted to images ");
}

#[tokio::main]
async fn main() {
    let router = Router::new().push(Router::new().path("/convert").post(convert_pdf));

    println!("🚀 Server running at http://127.0.0.1:5800/convert");

    let cors = Cors::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers("authorization")
        .into_handler();

    let service = Service::new(router).hoop(cors);
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;

    Server::new(acceptor).serve(service).await;
}
