package internal

import (
	"bufio"
	"os"
)

type Scanner struct {
	s *bufio.Scanner
	f *os.File
}

func NewScanner(path string) (*Scanner, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}

	s := bufio.NewScanner(f)

	scanner := &Scanner{s: s, f: f}

	return scanner, nil
}

func (s *Scanner) Scan() bool {
	if s != nil && s.s != nil {
		return s.s.Scan()
	}

	return false
}

func (s *Scanner) Text() string {
	if s != nil && s.s != nil {
		return s.s.Text()
	}

	return ""
}

func (s *Scanner) Err() error {
	if s != nil && s.s != nil {
		return s.s.Err()
	}

	return nil
}

func (s *Scanner) Close() {
	if s != nil && s.f != nil {
		s.f.Close()
	}
}

func OpenFile(path string) *os.File {
	f, err := os.Open(path)
	if err != nil {
		panic(err)
	}

	return f
}
