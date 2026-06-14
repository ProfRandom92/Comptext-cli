Output assets/demo/ctxt-demo.gif

Set Shell "powershell"
Set Width 960
Set Height 540
Set FontSize 16
Set FontFamily "Cascadia Mono"
Set Theme "Dracula"
Set Padding 18
Set TypingSpeed 25ms

Sleep 500ms
Type "cargo run --bin ctxt -- --json startup readiness"
Enter
Sleep 1s

Type "cargo run --bin ctxt -- --json capabilities"
Enter
Sleep 1s

Type "cargo run --bin ctxt -- --json review workflow"
Enter
Sleep 1s

Type "cargo run --bin ctxt -- --json validate --run"
Enter
Sleep 2s
