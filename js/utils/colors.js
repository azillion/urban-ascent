function generatePastelColor() {
  // Generate random RGB values within the pastel range
  var r = Math.floor(Math.random() * 128 + 128);
  var g = Math.floor(Math.random() * 128 + 128);
  var b = Math.floor(Math.random() * 128 + 128);

  // Convert RGB to hex
  var hex = ((r << 16) | (g << 8) | b).toString(16);

  // Prepend zeros if necessary
  while (hex.length < 6) {
    hex = '0' + hex;
  }

  // Return the pastel hex color code
  return '#' + hex;
}

export { generatePastelColor };