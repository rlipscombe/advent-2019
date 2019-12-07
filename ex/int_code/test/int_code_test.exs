defmodule IntCodeTest do
  use ExUnit.Case
  doctest IntCode

  test "greets the world" do
    assert IntCode.hello() == :world
  end
end
