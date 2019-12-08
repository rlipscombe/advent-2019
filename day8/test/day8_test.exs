defmodule Day8Test do
  use ExUnit.Case

  test "compose" do
    layers = [[0, 2, 2, 2], [1, 1, 2, 2], [2, 2, 1, 2], [0, 0, 0, 0]]
    assert Day8.compose_layers(layers) == [0, 1, 1, 0]
  end

  test "compose1" do
    layers = [[0, 2, 2, 2], [1, 1, 1, 1]]
    assert Day8.compose_layers(layers) == [0, 1, 1, 1]
  end
end
