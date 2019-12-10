package cmd

import "github.com/spf13/cobra"

import "fmt"

import "os"

var todo = &cobra.Command{
	Use:   "todo",
	Short: "manages todo lists",
}

func Execute() {
	if err := todo.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
