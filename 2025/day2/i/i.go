package i

func ValidId(id string) bool {
	if len(id)%2 != 0 {
		return true
	}

	mid := len(id) / 2

	start := id[:mid]
	end := id[mid:]

	return start != end
}

func CompileIds(start, end int) []int {
	ids := []int{}

	for i := start; i <= end; i++ {
		ids = append(ids, i)
	}

	return ids
}
