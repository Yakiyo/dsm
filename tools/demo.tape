# Generating a gif file using [vhs](https://github.com/charmbracelet/vhs)
# ./github/workflows/vhs.yml is used to keeping it up to date

Output tools/dsm.gif

Set FontSize 46
Set Width 1200
Set Height 600
Set WindowBar Colorful
Set Theme "TokyoNightStorm"

Type "eval `dsm env bash`"

Sleep 500ms

Enter

Sleep 500ms

Type "dsm install latest"

Enter

Type "dart --version"

Sleep 500ms

Enter

Sleep 2s
