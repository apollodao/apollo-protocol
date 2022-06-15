# Contribution guidelines

## Contributing a feature to `origin/master`
The basic workflow of contributing a feature to `origin/master`.
* Assign yourself to the issue and set it as "In progress" in [Apollo Kanban](https://github.com/orgs/apollodao/projects/2/views/1)
* Create a feature branch with an appropriate name by branching out from `origin/master`.
* Push your changes to you feature branch.
* When you are done. Clean up the commit history using `git rebase -i`
* Open a PR from your branch into `origin/master`
	* Link it to the issue it closes under "Development" in the right hand menu
	* Link it to the Apollo Kanban under "Projects" in the right hand menu
	* Request reviews from appropriate reviewers
* When all tests have passed and you have an Approval from code review you can merge your branch using "Merge with merge commit"
* After your PR is merged delete your feature branch.

During your work you might run into a number of situations for which we have special practices which follow below.

### My branch has conflicts and/or is out of sync with `origin/master`
When out of sync with master we work with a rebasing strategy. **DO NOT** merge `origin/master` into your feature branch.
* Checkout your branch. `git checkout <your-feature-branch>`
* Fetch the latest from upstream. `git fetch --all`
* Rebase your change on `origin/master`. `git rebase -i origin/master`

### Merging a PR into a larger open PR
Being multiple people on the same PR it is a good idea that all involved create their own feature branches branching out from the main feature branch.
When a mini feature branch is ready follow the same procedure as when merging to master but **DO NOT** user "Merge with merge commit".
Instead choose "Rebase and merge". That way when the feature branch gets merged to master our commit history will be cleaner.

### I need changes from another feature branch
To minimize git conflicts try to not copy files from other branches into your own.
If you need all or a majority of the changes in the other branch, consider rebasing on that feature branch and creating your PR into that branch instead
following the instructions above.
If you only need part of the changes see if you can cherry-pick the commits you need into your branch. `git cherry-pick <commit-id>`. 
If that does not work see if you can open a new PR to merge the needed commits from the other feature branch into `origin/master` and the rebase your branch on `origin/master`.
If none of the above work you may copy the changes over but beware that it is your karma to bear.



