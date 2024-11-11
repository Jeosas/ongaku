set positional-arguments

# Show this message
@default:
  just --unstable --list

@run *args='':
  RUSTFLAGS="-Adead_code -Aunused_variables" cargo run -q -- "$@"
