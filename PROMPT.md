# PROMT

rewriting "https://github.com/fastnear/explorer-frontend" in rust dioxus ui framework

a few key differences we will support both mainnet and testnet on same domain we use /testnet/tx /testnet/account, and so on and /mainnet/tx /mainnet/account and so on.
see network.rs file and network button for how i want to implemnet network settings.
would also be cool if for near accounts entered in the search bar if they end if .testnet auto switch to testnet, if they end in .near .tg or anything else auto switch to mainnet

this is our color palette
```palette
#8CA2F5
#C9A8F4
#95D58D
#FF8A8A
#FFFFFF
#000000
#FFF8A3
#FFC58A
```
we will just have light mode cause i don't have a color palette for dark mode unless you want to change this a little to make work for dark mode as well.

i want you to keep the explorer api client and other logic separte from ui components, and i want you to keep everything clean and organized. so that i can publish this crate and reuse

i have restructured this dioxus project a little i want you to see how i have files and keep the fromate the same way with the ```// ========``` comments that i have near top and bottom of file.


also see [AGENTS.md](./AGENTS.md)


---

copyright 2026 by sleet.near