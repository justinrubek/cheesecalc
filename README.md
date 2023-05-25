# cheesecalc

https://justinrubek.github.io/cheesecalc/

This is a quick and dirty calculator for preparing mac and cheese using ratios of sodium citrate, liquid, cheese, and pasta.

For convenience this is packaged as a nix flake.
Use `nix run github:justinrubek/cheesecalc`.
Alternatively clone the repo and use `nix run` directly within the project.

The program has a help page `nix run . -- --help`

![image](https://user-images.githubusercontent.com/25621857/176759443-f9053164-cd1e-41d0-8f6e-9c5af6c0b5a4.png)


## example usage

I want to serve 2 servings of mac and cheese with 100g of pasta in each.
Invoke the command: `nix run . -- --pasta 200`
```
➜ nix run . -- -p 200
cheese: 238.0952380952381g
liquid: 221.42857142857144ml
pasta: 200g
sodium citrate: 9.523809523809524g
```

Unfortunately I only have 200g of cheese on hand, so I will have to reduce the amounts of the other ingredients by using cheese as the base:
```
➜ nix run . -- -c 200
cheese: 200g
liquid: 186ml
pasta: 168g
sodium citrate: 8g
```
