package core

import (
	"crypto/sha1"
	"encoding/json"
	"fmt"

	"github.com/eropsyren/todo/constants"
)

// TaskList is a todo list
type TaskList struct {
	tasks []Task
}

func NewTaskList() *TaskList {
	return &TaskList{
		tasks: make([]Task, 0),
	}
}

// MarshalJSON converts TaskList into json
func (tl *TaskList) MarshalJSON() ([]byte, error) {
	res, err := json.Marshal(struct {
		Tasks []Task
	}{
		Tasks: tl.tasks,
	})

	if err != nil {
		return nil, err
	}

	return res, nil
}

// Task represents a task inside a todo list
type Task struct {
	id          string
	title       string
	description string
	status      string
}

func NewTask(title, description string) *Task {
	h := sha1.New()
	h.Write([]byte(title))

	return &Task{
		id:          fmt.Sprintf("%x", h.Sum(nil)),
		title:       title,
		description: description,
		status:      constants.Undone,
	}
}

// MarshalJSON converts Task into json
func (t *Task) MarshalJSON() ([]byte, error) {
	res, err := json.Marshal(struct {
		title  string
		descr  string
		status string
	}{
		title:  t.title,
		descr:  t.description,
		status: t.status,
	})

	if err != nil {
		return nil, err
	}

	return res, nil
}
