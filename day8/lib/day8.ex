defmodule Day8 do
  def main([file, width, height]) do
    pixels =
      File.read!(file) |> String.trim() |> String.to_charlist() |> Enum.map(&Kernel.-(&1, ?0))

    width = String.to_integer(width)
    height = String.to_integer(height)

    count = div(length(pixels), width * height)
    IO.puts("#{count} layers")
    layers = Enum.chunk_every(pixels, width * height)

    dump_layers(layers, width, height)

    # Find the layer containing the fewest zero digits
    layer = Enum.min_by(layers, &count(&1, 0))
    dump_layer(layer, width)

    count(layer, 1) |> IO.inspect()
    count(layer, 2) |> IO.inspect()

    result = count(layer, 1) * count(layer, 2)
    IO.puts("result: #{result}")

    final = compose_layers(layers)
    dump_layer(final, width)
  end

  defp count(layer, v) do
    Enum.count(layer, fn x -> x == v end)
  end

  defp dump_layers(layers, width, _height) do
    for layer <- layers do
      dump_layer(layer, width)
    end
  end

  defp dump_layer(layer, width) do
    rows = Enum.chunk_every(layer, width)

    for row <- rows do
      IO.puts(row |> Enum.map(&Kernel.+(&1, ?0)))
    end

    IO.puts("")
  end

  def compose_layers(layers) do
    # Layers are given from front to back.
    # 0 is black, 1 is white, 2 is transparent
    Enum.reduce(
      layers,
      fn next, curr ->
        Enum.zip(next, curr)
        |> Enum.map(fn
          # 0, 1 => take current
          {_b, 0} -> 0
          {_b, 1} -> 1
          # 2 => take next
          {b, 2} -> b
        end)
      end
    )
  end
end
