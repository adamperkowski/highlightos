# Core structure
## Prompt input
Current buffer can be interrupted by pressing <kbd>F1</kbd>.

The shell keeps a history of all commands executed.<br>History can be browsed with the `history` command or <kbd>&#8593;</kbd> and <kbd>&#8595;</kbd> keys.
## Command return codes
* `0`  >  executed successfully
* `1`  >  "silent" return code - doesn't show the message
* `2`  >  returned general error (minor)
* `3`  >  returned critical error (could not recover)
* `4`  >  returned user error (ex. incorrect input)

## Colors
**Currently available colors (user):**
* `black`
* `blue`
* `green`
* `cyan`
* `red`
* `magenta`
* `brown`
* `lightgray`
* `lightgreen`
* `lightcyan`
* `lightred`
* `yellow`
* `white`
