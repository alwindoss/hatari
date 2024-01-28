package engine

import (
	"fmt"
	"os"
)

// Commands:
// 	init --repo "path_to_dir" ==> initializes the repository
// 	backup --repo "path_to_dir" --source "path_to_dir" ==> Backs up the content in the source directory
// 	list repos ==> Lists all the repositories
// 	list snapshots --repo "path_to_dir" ==> Lists all the snapshots in the repository
// 	restore --repo "path_to_dir" --target "path_to_dir" --snapshot "snapshot_id" ==> Restores the snapshot to the target directory

type CreateBackupInput struct {
	Repository string
}

func CreateBackup() {
}

type InitalizeRepositoryInput struct {
	Repository string
}

func InitalizeRepository(inp InitalizeRepositoryInput) error {
	err := os.MkdirAll(inp.Repository, 0750)
	if err != nil {
		err = fmt.Errorf("unable to create the repository: %w", err)
		return err
	}
	// TODO: Do other things necessary for the clean functioning of the repository
	return nil
}

type ListInput struct {
}

func List() {

}

type RestoreBackupInput struct {
}

func RestoreBackup() {

}
