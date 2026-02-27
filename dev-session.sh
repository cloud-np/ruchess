#!/bin/bash
SESSION="dev"

tmux kill-session -t $SESSION 2>/dev/null

tmux new-session -d -s $SESSION -c "$HOME/Programming/my-repos/ruchess"
tmux send-keys -t $SESSION:0 "nvim ." C-m

tmux new-window -t $SESSION -c "$HOME/Programming/my-repos/ruchess"
tmux send-keys -t $SESSION:1 "claude" C-m

tmux new-window -t $SESSION -c "$HOME/Programming/repos/cozy-chess/"
tmux send-keys -t $SESSION:2 "nvim ." C-m

tmux new-window -t $SESSION -c "$HOME/Programming/repos/cozy-chess"
tmux send-keys -t $SESSION:3 "claude" C-m

tmux new-window -t $SESSION -c "$HOME/.config/nvim"
tmux send-keys -t $SESSION:4 "nvim ." C-m

tmux new-window -t $SESSION -c "$HOME/.config/nvim"
tmux send-keys -t $SESSION:5 "claude" C-m

tmux select-window -t $SESSION:0
tmux attach -t $SESSION
