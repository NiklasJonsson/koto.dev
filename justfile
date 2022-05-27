docs:
  cargo run -p convert-koto-docs

watch-docs:
  cargo watch -w modules/koto/docs -w tools/convert-koto-docs -x "run -p convert-koto-docs"

serve:
  @just docs && zola serve --open --fast
