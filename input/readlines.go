package input

import (
  "bufio"
  "os"
)

func Readlines(filename string) (fileLines []string, err error) {
  readFile, err := os.Open(filename)

  if err != nil {
    return nil, err
  }

  fileScanner := bufio.NewScanner(readFile)

  fileScanner.Split(bufio.ScanLines)

  for fileScanner.Scan() {
    fileLines = append(fileLines, fileScanner.Text())
  }

  readFile.Close()

  return fileLines, nil
}