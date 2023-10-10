```mermaid

graph TB;
	prog --> stmt[stmt^*]
	stmt --> exit["`_exit_([expr]);`"]
	stmt --> let["`_let_ ident = [expr];`"]
	exit --> expr
	let --> expr
	expr --> int_lit
	expr --> ident

```
