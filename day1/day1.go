package day1

func getDigit(char byte) (isDigit bool, num int) {
  digit := int(char - '0')
  if digit < 0 || digit > 9 {
    return false, 0
  }
  return true, digit
}

func Q1(input []string) (res int, err error) {
	res = 0

	for _, line := range input {
		i := 0

    for i < len(line) {
			isDigit, num := getDigit(line[i])
      if(isDigit) {
        res += 10*num
        break
      }
			i+=1
		}

    i = len(line)-1
    for i >= 0 {
      isDigit, num := getDigit(line[i])
      if(isDigit) {
        res += num
        break
      }
      i-=1
    }

	}

	return res, err
}

func Q2(input []string) (res int, err error) {
  res = 0
  words := [...]string {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

  for _, line := range input {
    i := 0

    outer:
    for i < len(line) {
      isDigit, num := getDigit(line[i])
      if(isDigit) {
        res += 10*num
        break
      }

      for pos, word := range words {
        if(i+len(word) <= len(line) && line[i:i+len(word)] == word) {
          res += 10 * (pos+1)
          break outer
        }
      }
      
      i+=1
    }

    i = len(line)-1
    
    outer2:
    for i >= 0 {
      isDigit, num := getDigit(line[i])
      if(isDigit) {
        res += num
        break
      }
      
      for pos, word := range words {
        if(i+len(word) <= len(line) && line[i:i+len(word)] == word) {
          res += pos+1
          break outer2
        }
      }
     
      i-=1
    }

  }

  return res, err
}
