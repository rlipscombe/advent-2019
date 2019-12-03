defmodule Day3 do
  def paths_from_file(f) do
    [a, b] = File.read!(f) |> String.split("\n", trim: true)
    {a, b}
  end

  def path_from_string(s) do
    String.split(s, ",") |> Enum.map(&to_step/1)
  end

  defp to_step("R" <> n), do: {:r, String.to_integer(n)}
  defp to_step("D" <> n), do: {:d, String.to_integer(n)}
  defp to_step("U" <> n), do: {:u, String.to_integer(n)}
  defp to_step("L" <> n), do: {:l, String.to_integer(n)}

  @doc """
  Given two paths, work out where the closest intersection to the start is.
  """
  def get_closest_intersection_for_paths(path_a, path_b) do
    # Work out which points each path visits.
    {points_a, _} = get_points_visited_by_path(path_a)
    {points_b, _} = get_points_visited_by_path(path_b)

    # Work out where they intersect.
    intersections = get_shared_points(points_a, points_b)

    # Which of those is closest to the origin?
    intersections
    |> MapSet.delete({0, 0})
    |> Enum.map(&manhattan/1)
    |> Enum.sort()
    |> Enum.at(0)
  end

  def get_points_visited_by_path(path) do
    get_points_visited_by_path(path_from_string(path), {0, 0, 0}, %{})
  end

  defp get_points_visited_by_path([step | rest], pos0, visited0) do
    {visited, pos} = do_step(step, pos0, visited0)
    get_points_visited_by_path(rest, pos, visited)
  end

  defp get_points_visited_by_path([], pos, visited), do: {visited, pos}

  defp do_step({:l, n}, {x, y, c}, visited0) do
    visited = Enum.reduce(0..n, visited0, fn i, v -> Map.put(v, {x - i, y}, c + i) end)

    {visited, {x - n, y, c + n}}
  end

  defp do_step({:r, n}, {x, y, c}, visited0) do
    visited = Enum.reduce(0..n, visited0, fn i, v -> Map.put(v, {x + i, y}, c + i) end)

    {visited, {x + n, y, c + n}}
  end

  defp do_step({:u, n}, {x, y, c}, visited0) do
    visited = Enum.reduce(0..n, visited0, fn j, v -> Map.put(v, {x, y + j}, c + j) end)

    {visited, {x, y + n, c + n}}
  end

  defp do_step({:d, n}, {x, y, c}, visited0) do
    visited = Enum.reduce(0..n, visited0, fn j, v -> Map.put(v, {x, y - j}, c + j) end)

    {visited, {x, y - n, c + n}}
  end

  defp get_shared_points(a, b) do
    ka = a |> Map.keys |> Enum.into(%MapSet{})
    kb = b |> Map.keys |> Enum.into(%MapSet{})
    MapSet.intersection(ka, kb)
  end

  defp manhattan({x, y}) do
    abs(x) + abs(y)
  end
end
