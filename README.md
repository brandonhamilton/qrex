# QRex

[![Hex.pm](https://img.shields.io/hexpm/v/qrex.svg)](https://hex.pm/packages/qrex)
[![Hex Docs](https://img.shields.io/badge/hex-docs-blue.svg)](https://hexdocs.pm/qrex)
[![License](https://img.shields.io/hexpm/l/qrex.svg)](https://github.com/brandonhamilton/qrex/blob/master/LICENSE)

QRex is a high-performance QR code detection and decoding library for Elixir, powered by Rust NIFs.

## Features

- Fast QR code detection and decoding
- Support for multiple QR codes in a single image
- Pre-compiled NIFs for major platforms
- Simple, straightforward API
- Detailed error reporting

## Installation

Add `qrex` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:qrex, "~> 0.1.0"}
  ]
end
```

## Usage

### Basic Example

```elixir
# Read an image file
image_bytes = File.read!("path/to/image.png")

# Detect and decode QR codes
case QRex.detect_qr_codes(image_bytes) do
  {:ok, results} ->
    # Process each detected QR code
    Enum.each(results, fn
      {:ok, qr_code} ->
        IO.puts "Found QR code: #{qr_code.text}"
        IO.puts "Version: #{qr_code.version}"
        IO.puts "Bounds: #{inspect(qr_code.bounds)}"
      
      {:error, reason} ->
        IO.puts "Failed to decode QR code: #{reason}"
    end)
    
  {:error, reason} ->
    IO.puts "Failed to process image: #{reason}"
end
```

### QR Code Information

The decoded QR code struct (`QRex.QRCode`) contains:

- `text` - The decoded content
- `version` - QR code version (1-40)
- `modules` - Size in modules (21-177)
- `ecc_level` - Error correction level (0-3)
- `bounds` - Corner coordinates as `[{x, y}, ...]`

## Platform Support

QRex provides pre-compiled NIFs for:

- Linux (x86_64, aarch64)
- macOS (x86_64, arm64)
- Windows (x86_64)

For other platforms, the library will attempt to compile from source. You'll need:

- Rust toolchain (1.79.0 or later)
- C compiler
- Development headers

## Building from Source

To force building from source, set the environment variable:

```bash
export RUSTLER_PRECOMPILATION_QREX_BUILD=1
mix deps.compile qrex --force
```

## Performance

QRex is designed for high performance, leveraging Rust's speed and safety features:

- Fast image processing algorithms
- Zero-copy data handling where possible
- Efficient memory management

## License

Licensed under:

- MIT license ([LICENSE](LICENSE))

---
