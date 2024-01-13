package main

import (
  "fmt"
  "main/input"
  d "main/day6"
)

func main() {
  lines, err := input.Readlines("day6/input.txt")
  if err != nil {
    fmt.Println(err)
  }
  res, err := d.Q1(lines)
  if err != nil {
    fmt.Println(err)
  }
  fmt.Println("Result: ", res)
}
