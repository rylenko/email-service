/// Makes a QR code from the `data`, then encodes it in PNG format in the
/// buffer.
///
/// # Panic
///
/// If `data` is too long.
#[must_use]
pub(super) fn make_png_bytes<T: AsRef<[u8]>>(data: T) -> Vec<u8> {
	use image::ImageEncoder as _;

	// Build QR code image
	let qrcode = qrcode::QrCode::new(data).unwrap();
	let image = qrcode.render::<image::Luma<u8>>().build();

	// Extract QR code image info
	let (width, height) = image.dimensions();
	let bytes = image.into_raw();

	// Encode QR code image into PNG
	let mut buffer = vec![];
	let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
	encoder.write_image(&bytes, width, height, image::ColorType::L8).unwrap();
	buffer
}
