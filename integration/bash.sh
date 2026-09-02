bkp() {
  finder-rs places get $1
}

replace_with_output() {
  local cmd="$READLINE_LINE"
  READLINE_LINE="$(eval "$cmd")"
  READLINE_POINT=${#READLINE_LINE}
}

bind -x '"\C-x\C-b":replace_with_output'
