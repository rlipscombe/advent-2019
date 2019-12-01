defmodule Day1aTest do
  use ExUnit.Case
  doctest Day1a

  test "basic numbers" do
    assert Day1a.get_fuel_for_module(12) == 2
    assert Day1a.get_fuel_for_module(14) == 2
    assert Day1a.get_fuel_for_module(1969) == 654
    assert Day1a.get_fuel_for_module(100756) == 33583
  end
end
