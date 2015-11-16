# Contributors Guideline

We're very happy if people contribute to our projects! To keep quality high,
please adhere to the following standards though:

- **Always create a fork or branch** if you develop a new feature or fix. Then
  create a pull request against `master` to get the changes integrated.
- **Run the tests** (`cargo test`) before committing and pushing your changes.
- **Write meaningful commit messages**: First line of your commit message should be
  a very short summary in the imperative mood without any punctuation symbols
  (ideally 50 characters or less). After the first line of the commit message,
  add a blank line and then a more detailed explanation (when relevant).
  [This](http://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html) is
  a nice blog post concerning git commit messages.
- **Keep your commit history clean**. If there are typo commits that should be squashed
  or if you want to re-order commits, please use `git rebase -i` or `git commit --amend`
  to clean up your commit history.

If you're a member of the development team (having push access to the repository):

- Create a pull request, then wait for another member of the team to review
  your changes. Only merge your PR if you got a "good to merge" comment.
- Don't merge the pull requests of other members of the development team.
  Instead, let them know that you think their contribution is good to merge.

If you're an outside contributor:

- Simply create a pull request and wait for a team member to merge it.
