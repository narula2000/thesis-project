let mapleader=","

filetype plugin indent on
syntax on

set pumheight=10
set splitright
set guicursor=n:blinkon0
set relativenumber
set nohlsearch
set hidden
set noerrorbells
set tabstop=4 softtabstop=4
set shiftwidth=4
set expandtab
set smartindent
set nu 
set nowrap
set ignorecase
set smartcase
set noswapfile
set nobackup
set nowritebackup
set undofile
set incsearch
set termguicolors
set scrolloff=8
set noshowmode
set cursorline
set clipboard+=unnamedplus
set encoding=UTF-8
set cmdheight=2
set updatetime=50
set shortmess+=c
set wildmode=longest,list,full

" Visual Ruler
set colorcolumn=81,121
highlight ColorColumn ctermbg=0 guibg=black

colorscheme koehler

" Remap tank whole line to yank till end of line
nnoremap Y y$

" Moving Text
vnoremap J :m '>+1<CR>gv=gv
vnoremap K :m '<-2<CR>gv=gv

" Vertical resize
nnoremap <Leader>+ :vertical resize +5<CR>
nnoremap <Leader>- :vertical resize -5<CR>

" Undo till last changes
noremap U :earlier 1f<CR>

" Shortcutting split navigation, saving a keypress:
map <C-h> <C-w>h
map <C-j> <C-w>j
map <C-k> <C-w>k
map <C-l> <C-w>l

" Chang j k to move by visual line
nnoremap j gj
nnoremap k gk

" Change buffer and quit buffer
nnoremap <leader>q :bd!<CR>
nnoremap <TAB> :bn<CR>
nnoremap <S-TAB> :bp<CR>

" Use Esc to exit Insert mode in Terminal
tnoremap <Esc> <C-\><C-n>
