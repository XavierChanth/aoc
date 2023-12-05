package main

import (
  "fmt"
  "main/input"
  d "main/day2"
)

func main() {
  lines, err := input.Readlines("day2/input.txt")
  if err != nil {
    fmt.Println(err)
  }
  res, err := d.Q2(lines)
  if err != nil {
    fmt.Println(err)
  }
  fmt.Println("Result: ", res)
}
