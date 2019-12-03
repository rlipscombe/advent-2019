defmodule Day3Test do
  use ExUnit.Case
  doctest Day3

  test "path from string" do
    assert Day3.path_from_string("R75,D30,R83,U83,L12") == [
             {:r, 75},
             {:d, 30},
             {:r, 83},
             {:u, 83},
             {:l, 12}
           ]
  end

  test "get_points_visited_by_path 1" do
    assert {_, {8,0}} = Day3.get_points_visited_by_path("R8")
  end

  test "get_points_visited_by_path 2" do
    assert {_, {8,5}} = Day3.get_points_visited_by_path("R8,U5")
  end

  test "get_points_visited_by_path 3" do
    assert {_, {3,5}} = Day3.get_points_visited_by_path("R8,U5,L5")
  end

  test "get_points_visited_by_path 4" do
    assert {_, {3,2}} = Day3.get_points_visited_by_path("R8,U5,L5,D3")
  end

  test "negative points" do
    assert {_, {-3, -6}} = Day3.get_points_visited_by_path("R1,U1,L4,D7")
    assert Day3.get_closest_intersection_for_paths("R1,U1", "U1,R1") == 2
  end

  test "first example" do
    assert Day3.get_closest_intersection_for_paths(
             "R75,D30,R83,U83,L12,D49,R71,U7,L72",
             "U62,R66,U55,R34,D71,R55,D58,R83"
           ) == 159
  end

  test "second example" do
    assert Day3.get_closest_intersection_for_paths(
             "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
             "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
           ) == 135
  end

  test "puzzle" do
    {a, b} = Day3.paths_from_file("test-data.txt")
    assert Day3.get_closest_intersection_for_paths(a, b) == 489
  end
end
