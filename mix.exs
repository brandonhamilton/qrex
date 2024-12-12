defmodule QRex.MixProject do
  use Mix.Project

  @version "0.1.0"
  @source_url "https://github.com/brandonhamilton/qrex"

  def project do
    [
      app: :qrex,
      version: @version,
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      description: "QR code detector and decoder",
      source_url: @source_url,
      package: package(),
      deps: deps(),
      docs: docs()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler_precompiled, "~> 0.8"},
      # optional
      {:rustler, ">= 0.0.0", optional: true},
      {:ex_doc, "~> 0.34.2", only: :dev, runtime: false}
    ]
  end

  defp docs() do
    [
      main: "QRex",
      extras: ["README.md", "LICENSE"]
    ]
  end

  defp package() do
    [
      files: [
        "lib",
        "native/qrex/.cargo",
        "native/qrex/src",
        "native/qrex/Cargo*",
        "checksum-*.exs",
        "mix.exs",
        "README.md",
        "LICENSE"
      ],
      licenses: ["MIT"],
      links: %{"GitHub" => @source_url}
    ]
  end
end
