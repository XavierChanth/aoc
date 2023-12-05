package day2

import (
	sc "strconv"
	s "strings"
)

func Q1(input []string) (res int, err error) {
	res = 0
	rmax, gmax, bmax := 12, 13, 14

outer:
	for _, line := range input {
		parts := s.Split(line, ":")
		games := s.Split(parts[1], ";")

		for _, game := range games {
			r, g, b := rmax, gmax, bmax
			draws := s.Split(game, ",")

			for _, draw := range draws {
				draw := s.Split(draw, " ") // draw[0] is empty string
				count, err := sc.Atoi(draw[1])
				if err != nil {
					return 0, err
				}

				switch draw[2] {
				case "red":
					r -= count
					break
				case "green":
					g -= count
					break
				case "blue":
					b -= count
					break
				}
			}

			if r < 0 || g < 0 || b < 0 {
				continue outer
			}
		}

		id, err := sc.Atoi(s.Split(parts[0], " ")[1])
		if err != nil {
			return 0, err
		}

		res += id
	}
	return res, nil
}

func Q2(input []string) (res int, err error) {
	res = 0

	for _, line := range input {
		parts := s.Split(line, ":")
		games := s.Split(parts[1], ";")
		rmax, gmax, bmax := 0, 0, 0
		for _, game := range games {
			r, g, b := 0, 0, 0
			draws := s.Split(game, ",")

			for _, draw := range draws {
				draw := s.Split(draw, " ") // draw[0] is empty string
				count, err := sc.Atoi(draw[1])
				if err != nil {
					return 0, err
				}

				switch draw[2] {
				case "red":
					r += count
					break
				case "green":
					g += count
					break
				case "blue":
					b += count
					break
				}
			}

			if r > rmax {
				rmax = r
			}

			if g > gmax {
				gmax = g
			}

			if b > bmax {
				bmax = b
			}
		}


		res += rmax * gmax * bmax
	}

	return res, nil
}