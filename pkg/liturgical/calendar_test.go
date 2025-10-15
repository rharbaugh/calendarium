package liturgical

import "testing"

func TestNewCalendar(t *testing.T) {
	cal := NewCalendar()
	if cal == nil {
		t.Error("NewCalendar returned nil")
	}
}
