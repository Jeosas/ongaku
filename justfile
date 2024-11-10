set positional-arguments

# Show this message
@default:
  just --unstable --list

@run *args='':
  cargo run -q -- "$@"
