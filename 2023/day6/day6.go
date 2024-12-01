package day6

import (
	"fmt"
	sc "strconv"
	s "strings"
)

func Q1(input []string) (res int, err error) {
	res = 1
	times := s.Fields(input[0])
	dists := s.Fields(input[1])

	for i, timestring := range times {
		if i == 0 {
			continue
		}
		count := 0

		time, err := sc.Atoi(timestring)
		if err != nil {
			return 0, err
		}

		dist, err := sc.Atoi(dists[i])
		if err != nil {
			return 0, err
		}

		for x := 0; x <= time; x++ {
			fmt.Println(x, (time - x), x*(time-x), dist, count)
			if x*(time-x) > dist {
				count += 1
			} else if count > 0 {
				break
			}
		}
		fmt.Println(count)
		res *= count
	}

	return res, nil
}
