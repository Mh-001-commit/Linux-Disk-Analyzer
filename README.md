
## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


The objective of the disk analyzer program is to scan the file system structure and graphically represent the
directories entries with emphasis on the size occupied by each entry. This tool is interactive and
visually represents an accurate and efficient way to understand your Disk Utilization.

Main Features
● File Management
○ Access directory
○ Delete files
● Sorting in the tables based on
○ Files with the largest size in the current directory
○ Oldest files
○ File with the most number of underlying files and directories
● Sunburst Chart
○ Zoom in and out among directories
○ Visualize directories in an interactive way
● Nested Data Tree
○ Show all attributes of file/folder
○ delete a file/folder
○ start a new scan directly


Back-end
● Data Structures:
The main data structure is the struct “PathNode” which represents either a file or
folder in the file system. List of attributes of “PathNode” :
● name: String, (name of the node)
● value: i128, (size of the node)
● relative_path: String, (the path of the node relative to the analyzing path)
● absolute_path: String, (the full path of the node)
● UserID: u32, (the owner ID)
● actime: NaiveDateTime, (last access time)
● modtime: NaiveDateTime, (last modification time)
● node_type: NodeType, (file or directory)
● children: (folders or files contained in this node)
● Traversal Algorithm :
The algorithm starts by extracting the attributes of the root node. Then the
path is passed to a function that recursively traverses the directories and files to
collect their attributes and construct the hierarchical tree of nodes. Using
deserialze, the tree is converted into a JSON object and passed to the front end.
Front-end
● HTML
○ HTML was used to create the skeleton of the application.
○ It helps to add items to the app: chart, tree, titles, and buttons
● CSS
○ CSS was used in all styling
○ It helps in managing spaces and colors
○ CSS adds spirit to the app and make it more appealing
● JavaScript
○ JS is the main programming language used to create the front end
functions.

Integration
● Tauri
○ Tauri help us to launch a desktop application using HTML, CSS, and Java
script while using RUST as our back-end language
● JSON
○ It’s the file type used to dynamically update the data and parse it to the
front-end.
○ All the dynamics of the chart and the tree were done by JS.
○ It helps to make the application interactive

