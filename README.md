#   Use luma lib in nula
@ Example using luma-lib for beautiful CLI

<luma-lib>
>_ luma: print_colored, draw_frame, progress_bar, draw_table, Red, Green, Blue, Yellow

write "Starting Luma CLI Demo..."

@ Print colored text
luma.print_colored["Hello, Nula World!", luma.Red, none]

@ Draw a frame
let content = ["Line 1", "Line 2 with more text", "Line 3"]
luma.draw_frame[content, luma.Blue, "My Frame Title"]

@ Progress bar
luma.progress_bar[10, "Processing...", luma.Green]

@ Draw a table
let headers = ["Name", "Age", "City"]
let rows = [["Alice", "30", "New York"], ["Bob", "25", "San Francisco"]]
luma.draw_table[headers, rows, luma.Yellow]

write "Luma CLI Demo Complete!"
