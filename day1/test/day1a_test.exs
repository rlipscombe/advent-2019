defmodule Day1aTest do
  use ExUnit.Case
  doctest Day1a

  test "fuel for mass" do
    assert Day1a.get_fuel_for_mass(12) == 2
    assert Day1a.get_fuel_for_mass(14) == 2
    assert Day1a.get_fuel_for_mass(1969) == 654
    assert Day1a.get_fuel_for_mass(100756) == 33583
  end

  test "fuel for module" do
    assert Day1a.get_fuel_for_module(14) == 2
    assert Day1a.get_fuel_for_module(1969) == 966
    assert Day1a.get_fuel_for_module(100756) == 50346
  end

  test "final result" do
    Day1a.from_file("test-data.txt") |> IO.inspect
  end
end
