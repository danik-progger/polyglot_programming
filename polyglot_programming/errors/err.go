package main
import "fmt"

func returnError(val int) error {
	return fmt.Errorf("error %v ", val)
}

type Foo2 struct {}

func (f *Foo2) thisIsFoo() error {
	return fmt.Errorf("Foo 2")
}

func createFoo(fail bool) (*Foo2, error) {
	if fail {
		return nil, returnError(5)
	}
	return &Foo2{}, nil
}

func f() (*Foo2, error) {
	foo2, err := createFoo(false)

	if err != nil {
		return nil, err 
	}
	return foo2, nil
}