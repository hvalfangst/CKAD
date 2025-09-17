# Vim Configuration for CKAD

## 🎨 Essential .vimrc Setup

Add to `~/.vimrc`:

```vim
" Essential vim settings for YAML
set tabstop=2
set shiftwidth=2
set expandtab
set autoindent
set number
set ruler
set cursorline
syntax on

" YAML specific settings
autocmd FileType yaml setlocal ts=2 sts=2 sw=2 expandtab
autocmd FileType yaml setlocal autoindent
autocmd FileType yaml setlocal cursorcolumn

" Useful key mappings
nnoremap <F2> :set paste!<CR>
nnoremap <F3> :set number!<CR>
nnoremap <F4> :set list!<CR>

" Quick save and quit
nnoremap <C-s> :w<CR>
inoremap <C-s> <Esc>:w<CR>a

" Better navigation
nnoremap j gj
nnoremap k gk
```

## ⚡ Quick Vim Commands for YAML

### Indentation Commands
```vim
>>        # Indent line right (normal mode)
<<        # Indent line left (normal mode)
=         # Auto-indent selection (visual mode)
gg=G      # Auto-indent entire file
```

### Copy/Paste Operations
```vim
yy        # Copy current line
dd        # Delete (cut) current line
p         # Paste below cursor
P         # Paste above cursor
u         # Undo last change
Ctrl+r    # Redo
```

### Search and Replace
```vim
/pattern          # Search forward
?pattern          # Search backward
n                 # Next search result
N                 # Previous search result
:%s/old/new/g     # Replace all occurrences
:%s/old/new/gc    # Replace with confirmation
```

### Navigation
```vim
gg        # Go to beginning of file
G         # Go to end of file
:20       # Go to line 20
0         # Go to beginning of line
$         # Go to end of line
w         # Next word
b         # Previous word
```

## 🚨 Common YAML Issues & Fixes

### Paste Mode (CRITICAL!)
```vim
:set paste        # Enable paste mode before pasting
# Paste your content
:set nopaste      # Disable paste mode after pasting
```

### Show Hidden Characters
```vim
:set list         # Show tabs and spaces
:set nolist       # Hide tabs and spaces
```

### Fix Indentation Issues
```vim
# Select all text and fix indentation
gg         # Go to top
V          # Start visual line mode
G          # Select to bottom
=          # Auto-indent selection
```

## 🎯 YAML-Specific Tips

### Checking Syntax
```vim
# Check YAML syntax without leaving vim
:!kubectl apply --dry-run=client -f %

# Quick syntax check
:syntax on
```

### Working with Multiple Files
```vim
:e filename.yaml      # Open new file
:bn                   # Next buffer
:bp                   # Previous buffer
:ls                   # List all buffers
:b2                   # Switch to buffer 2
```

### Visual Block Mode (for bulk edits)
```vim
Ctrl+v      # Enter visual block mode
# Select column
I           # Insert at beginning of selection
# Type your text
Esc         # Apply to all selected lines
```

## ⏱️ Time-Saving Vim Shortcuts

### Quick File Operations
```vim
:w              # Save
:wq             # Save and quit
:q!             # Quit without saving
:x              # Save and quit (same as :wq)
ZZ              # Save and quit (normal mode)
```

### Efficient Editing
```vim
o               # Open new line below and enter insert mode
O               # Open new line above and enter insert mode
A               # Go to end of line and enter insert mode
I               # Go to beginning of line and enter insert mode
```

### Block Operations
```vim
# Comment multiple lines
Ctrl+v          # Visual block mode
# Select lines
I#<space>       # Insert comment
Esc             # Apply to all lines

# Uncomment multiple lines
Ctrl+v          # Visual block mode
# Select comment characters
x               # Delete selection
```

## 🔧 Quick Setup Command

One-liner to set essential vim settings:

```bash
echo -e "set tabstop=2\nset shiftwidth=2\nset expandtab\nset number\nsyntax on" >> ~/.vimrc
```

## 🚀 Vim Alternatives

If vim is challenging, these editors might be available:

```bash
nano filename.yaml    # Simple editor
vi filename.yaml      # Basic vi
```

But stick with vim for the exam - it's worth the investment!