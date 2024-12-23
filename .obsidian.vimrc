imap jk <Esc>
imap kj <Esc>
" Have j and k navigate visual lines rather than logical ones
nmap j gj
nmap k gk
" Navigate logical lines with J & K
noremap <C-j> j
noremap <C-k> k

noremap <C-d> <C-d>zz
noremap <C-u> <C-u>zz

" Yank to system clipboard
set clipboard=unnamed
unmap <C-c>
"
" Maps pasteinto to Alt-p
map <A-p> :pasteinto

" Go back and forward with Ctrl+O and Ctrl+I
" (make sure to remove default Obsidian shortcuts for these to work)
exmap back obcommand app:go-back
nmap <C-o> :back<CR>
exmap forward obcommand app:go-forward
nmap <C-i> :forward<CR>

exmap tabnext obcommand cycle-through-panes:cycle-through-panes
nmap gt :tabnext<CR>
exmap tabprev obcommand cycle-through-panes:cycle-through-panes-reverse
nmap gT :tabprev<CR>

exmap findfiles obcommand switcher:open
nmap ff :findfiles<CR>
exmap grepfiles obcommand global-search:open
nmap fg :grepfiles<CR>
