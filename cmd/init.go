package cmd

import (
	"log"
	"os"

	"github.com/eropsyren/todo/constants"
	"github.com/eropsyren/todo/core"
	"github.com/spf13/cobra"
)

func init() {
	todo.AddCommand(initCmd)
}

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize todo list",
	Run: func(cmd *cobra.Command, args []string) {
		taskList := core.NewTaskList()

		json, err := taskList.MarshalJSON()
		if err != nil {
			log.Fatal(err)
		}

		f, err := os.Create(constants.TodoFileName)
		if err != nil {
			log.Fatal(err)
		}

		defer func() {
			if err := f.Close(); err != nil {
				log.Fatal(err)
			}
		}()

		_, err = f.Write(json)
		if err != nil {
			log.Fatal(err)
		}

		if err = f.Sync(); err != nil {
			log.Fatal(err)
		}
	},
}
