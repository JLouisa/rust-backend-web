use printpdf::*;
use std::fs::File;
use std::io::{BufWriter, Cursor, Result, Write};

pub struct MyPdf(pub Vec<u8>);
impl MyPdf {
    pub fn get_pdf(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn test_create_pdf_file() -> Result<()> {
        // Create a new PDF document
        let (doc, page1, layer1) =
            PdfDocument::new("PDF Document Title", Mm(210.0), Mm(297.0), "Layer 1");

        // let font = doc
        //     .add_external_font(
        //         File::open("assets/fonts/Roboto-Regular.ttf").expect("Could not open font"),
        //     )
        //     .expect("Could not add font");

        let font = doc
            .add_builtin_font(BuiltinFont::Helvetica)
            .expect("Could not add font");

        // Get the first page's layer
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Text settings
        let text = "Hello Rust!";
        let font_size = 48.0; // size in points

        // Add text using direct coordinates
        current_layer.use_text(text, font_size, Mm(20.0), Mm(270.0), &font);

        // Save the PDF to a file
        doc.save(&mut BufWriter::new(
            File::create("pdf/test2.pdf").expect("Could not create file"),
        ))
        .expect("Couldn't save pdf");

        Ok(())
    }

    pub fn test_create_pdf_in_memory() -> Result<MyPdf> {
        // Create a new PDF document
        let (doc, page1, layer1) =
            PdfDocument::new("PDF Document Title", Mm(210.0), Mm(297.0), "Layer 1");
        let font = doc
            .add_builtin_font(BuiltinFont::Helvetica)
            .expect("Could not add font");

        // Get the first page's layer
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Text settings
        let text = "Hello Rust!";
        let font_size = 48.0; // size in points

        // Add text using direct coordinates
        current_layer.use_text(text, font_size, Mm(20.0), Mm(270.0), &font);

        // Prepare an in-memory buffer
        let mut buffer = Vec::new();
        let cursor = Cursor::new(&mut buffer);
        let mut writer = BufWriter::new(cursor);

        // Save the PDF to the buffer
        doc.save(&mut writer).expect("Could not save PDF to buffer");

        // Ensure all data is flushed into the buffer
        writer.flush()?;

        // Retrieve the inner Vec<u8> from the Cursor
        let buffer = writer.into_inner()?; // Retrieves the Cursor
        let buffer = buffer.into_inner(); // Retrieves the Vec<u8> from the Cursor
        let buffer = buffer.clone(); // Clones the Vec<u8>

        // Return the buffer containing the PDF data
        Ok(MyPdf(buffer))
    }
}

#[cfg(test)]
mod pdf_tests {
    use super::*;

    #[test]
    fn test_generate_pdf_operations() {
        let create_pdf = MyPdf::test_create_pdf_file();
        assert_eq!(create_pdf.is_ok(), true, "PDF not created successfully");

        let create_pdf = MyPdf::test_create_pdf_in_memory();
        assert_eq!(create_pdf.is_ok(), true, "PDF not created successfully");
    }
}
