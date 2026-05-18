check:
	@pnpm --silent run lint
	@cargo run --quiet --package skill-cli -- check
