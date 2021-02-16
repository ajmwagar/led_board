use rpi_led_matrix::{LedMatrix, LedColor, LedMatrixOptions};

fn main() {
	let mut options = LedMatrixOptions::new();
	options.set_hardware_mapping("adafruit-hat");
	let matrix = LedMatrix::new(Some(options), None).unwrap();
	let mut canvas = matrix.offscreen_canvas();
	for red in 0..255 {
	    for green in 0..255 {
		for blue in 0..255 {
		    canvas.fill(&LedColor { red, green, blue });
		    canvas = matrix.swap(canvas);
		}
	    }
	}
}
