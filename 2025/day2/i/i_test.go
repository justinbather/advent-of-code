package i

import "testing"

func TestValidId(t *testing.T) {
	id := "12212"
	act := ValidId(id)

	if act == true {
		t.Fatal("expected false")
	}
}

func TestCompileIds(t *testing.T) {

	ids := CompileIds(11, 15)

	expect := []int{11, 12, 13, 14, 15}

	if len(ids) != len(expect) {
		t.Fatalf("expected equal slices got: %v", ids)
	}
}
