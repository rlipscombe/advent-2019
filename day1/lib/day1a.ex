defmodule Day1a do
  @moduledoc """
  Implementation of Day 1 of Advent of Code, 2019.
  """

  def from_file(path) do
    modules =
      File.read!(path)
      |> String.split("\n", trim: true)
      |> Enum.map(&String.to_integer/1)

    modules
    |> Enum.map(&get_fuel_for_module/1)
    |> Enum.sum()
  end

  @doc """
  For a given module, work out how much fuel we need.
  """
  def get_fuel_for_module(mass) do
    # Starting with the mass of the module, get the fuel.
    fuel = get_fuel_for_mass(mass)

    # Iterate, getting the fuel for the fuel,
    # until we get to zero.
    # Sum.
    Stream.iterate(fuel, fn m -> get_fuel_for_mass(m) end)
    |> Stream.take_while(fn m -> m > 0 end)
    |> Enum.sum()
  end

  def get_fuel_for_mass(mass), do: div(mass, 3) - 2
end
