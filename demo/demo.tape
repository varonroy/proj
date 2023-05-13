Output demo.gif

Require proj
Require tree

Set Shell "zsh"
Set FontSize 30
Set Width 1024
Set Height 1024

# intro and list
Type "# List all projects."
Ctrl+C
Enter
Type "tree examples"
Enter 2
Sleep 1s

# add the projects
Type "# Mark all project direcotries."
Ctrl+C
Enter
Sleep 1s

# project: my-noets
Type "cd examples/personal/my-notes"
Enter
Type "proj --mark"
Enter 2

Sleep 500ms

# project: work-docs
Type "cd ../../work/work-docs"
Enter
Type "proj --mark"
Enter 2

Sleep 500ms

# project: work-project
Type "cd ../work-project"
Enter
Type "proj --mark"
Enter 2

Enter
Sleep 1s

# showcase the app
Type "# All the projects have been added."
Ctrl+C
Sleep 500ms
Type "# Open proj to quickly access any of them."
Ctrl+C
Sleep 500ms

# open
Type "proj"
Sleep 500ms
Enter
Sleep 800ms

# open the work-docs project
Type "w"
Sleep 300ms
Type "d"
Sleep 300ms
Type "o"
Sleep 300ms
Type "cs"
Sleep 1s
Enter

Sleep 1s
