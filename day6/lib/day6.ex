defmodule Day6 do
  def main([file]) do
    data =
      File.read!(file)
      |> String.trim()
      |> String.split("\n")
      |> Enum.map(fn s -> String.split(s, ")") end)
      |> IO.inspect()

    g =
      Enum.reduce(data, Graph.new(type: :directed), fn [a, b], g ->
        g |> Graph.add_vertex(a) |> Graph.add_vertex(b) |> Graph.add_edge(a, b)
      end)

    # dot -Tpng -o test-data.png test-data.dot
    # xdg-open test-data.png
    {:ok, dot} = Graph.to_dot(g)
    File.write!(Path.rootname(file) <> ".dot", dot)

    g = add_distances(g, "COM")

    result = Graph.Reducers.Dfs.reduce(g, 0, fn v, acc ->
      [d] = Graph.vertex_labels(g, v)
      IO.puts("#{v} has distance #{d}")
      {:next, acc + d}
    end)

    IO.puts result
  end

  defp add_distances(g, root) do
    add_distances(g, root, 0)
  end

  defp add_distances(g, v, d) do
    g = Graph.label_vertex(g, v, d)
    neighbours = Graph.out_neighbors(g, v)

    Enum.reduce(neighbours, g, fn n, g ->
      add_distances(g, n, d + 1)
    end)
  end
end
