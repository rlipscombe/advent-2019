defmodule Day6 do
  def main([verb, file]) do
    data =
      File.read!(file)
      |> String.trim()
      |> String.split("\n")
      |> Enum.map(fn s -> String.split(s, ")") end)

    g =
      Enum.reduce(data, Graph.new(type: :directed), fn [a, b], g ->
        g |> Graph.add_vertex(a) |> Graph.add_vertex(b) |> Graph.add_edge(a, b)
      end)

    case verb do
      "dot" ->
        # dot -Tpng -o test-data.png test-data.dot
        # xdg-open test-data.png
        {:ok, dot} = Graph.to_dot(g)
        File.write!(Path.rootname(file) <> ".dot", dot)

        System.cmd("dot", [
          "-Tpng",
          "-o",
          Path.rootname(file) <> ".png",
          Path.rootname(file) <> ".dot"
        ])

      "distances" ->
        g = add_distances(g, "COM")

        result =
          Graph.Reducers.Dfs.reduce(g, 0, fn v, acc ->
            [d] = Graph.vertex_labels(g, v)
            IO.puts("#{v} has distance #{d}")
            {:next, acc + d}
          end)

        IO.puts(result)

      "transfer" ->
        edges = Graph.edges(g)

        g =
          Enum.reduce(edges, g, fn e, g ->
            a = e.v1
            b = e.v2
            Graph.add_edge(g, b, a)
          end)

        vs = Graph.get_shortest_path(g, "YOU", "SAN") |> IO.inspect()
        result = length(vs) - 3
        IO.puts(result)
    end
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
