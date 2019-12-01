defmodule Day1a do
  @moduledoc """
  Implementation of Day, Puzzle 1 of Advent of Code, 2019.
  """

  def from_file(path) do
    File.read!(path)
    |> String.split("\n", trim: true)
    |> Enum.map(&String.to_integer/1)
    |> Enum.map(&get_fuel_for_module/1)
    |> Enum.sum
  end

  @doc """
  For a given module, work out how much fuel we need.
  """
  def get_fuel_for_module(mass) do
    div(mass, 3) - 2
  end
end
