defmodule QRex.QRCode do
  @moduledoc """
  Represents the decoded data and metadata of a QR code.

  This struct contains all the essential information extracted from a QR code,
  including its content, version, size, error correction level, and position
  in the image.

  ## Fields

    * `:text` - The decoded textual content of the QR code
    * `:version` - QR code version number (1-40)
    * `:modules` - Size of the QR code in modules (21-177)
    * `:ecc_level` - Error correction level (0-3)
    * `:bounds` - List of four coordinate pairs representing the QR code's corners

  ## Examples

      %QRex.QRCode{
        text: "Hello, World!",
        version: 1,
        modules: 21,
        ecc_level: 0,
        bounds: [{10, 10}, {31, 10}, {31, 31}, {10, 31}]
      }
  """
  defstruct text: nil,
            version: 1,
            modules: 21,
            ecc_level: 0,
            bounds: []

  @typedoc """
  QR code data structure.

  Contains the following fields:
    * `:text` - The textual content of the QR code
    * `:version` - The version of the QR code (1-40)
    * `:modules` - The number of modules in the QR code (21-177)
    * `:ecc_level` - The error correction level (0-3)
    * `:bounds` - The four boundary points of the QR code as coordinate pairs
  """
  @type t :: %__MODULE__{
          text: String.t(),
          version: pos_integer(),
          modules: pos_integer(),
          ecc_level: non_neg_integer(),
          bounds: list({integer(), integer()})
        }
end

defmodule QRex do
  @moduledoc """
  QRex is a high-performance QR code detection and decoding library.

  This module provides functionality to detect and decode QR codes from image data
  using Rust-based NIFs (Native Implemented Functions).

  ## Usage

      # Detect QR codes in an image
      image_bytes = File.read!("qr_code.png")
      {:ok, decoded_codes} = QRex.detect_qr_codes(image_bytes)

  The library supports precompiled NIFs for various platforms and will automatically
  use the appropriate version for your system.

  ## Error Handling

  The detection function returns results in the following format:
    * `{:ok, [{:ok, qr_code}, ...]}` - Successful detection with decoded QR codes
    * `{:ok, [{:error, reason}, ...]}` - Successful detection with some decode errors
    * `{:error, reason}` - Failed to process the image

  ## Environment Variables

    * `RUSTLER_PRECOMPILATION_QREX_BUILD` - Set to "1" or "true" to force building from source
  """
  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :qrex,
    crate: "qrex",
    base_url: "https://github.com/brandonhamilton/qrex/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_QREX_BUILD") in ["1", "true"],
    version: version

  @doc """
  Detects and decodes QR codes in the provided image data.

  Takes raw image bytes as input and attempts to locate and decode any QR codes
  present in the image.

  ## Parameters

    * `image_bytes` - Binary data of the image file

  ## Returns

    * `{:ok, codes}` where `codes` is a list of results:
      * `{:ok, qr_code}` for successfully decoded QR codes
      * `{:error, reason}` for QR codes that couldn't be decoded
    * `{:error, reason}` if image processing fails

  ## Examples

      iex> image_data = File.read!("test.png")
      iex> {:ok, results} = QRex.detect_qr_codes(image_data)
      iex> results
      [{:ok, %QRex.QRCode{text: "Hello!", version: 1, ...}}]
  """
  @spec detect_qr_codes(binary()) ::
          {:ok, [{:ok, QRex.QRCode.t()} | {:error, String.t()}]} | {:error, String.t()}
  def detect_qr_codes(_image_bytes), do: :erlang.nif_error(:nif_not_loaded)
end
