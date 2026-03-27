# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── carbon_tracker
│       ├── src
│       │   ├── lib.rs        # Smart contract logic
│       │   └── test.rs       # Unit tests
│       └── Cargo.toml
├── Cargo.toml               # Workspace configuration
└── README.md
```
<img width="1915" height="933" alt="image" src="https://github.com/user-attachments/assets/a3d51101-ad82-4985-b781-c2d79dd6cf9d" />
CBQ6VRZMMO47A2TPAIHZXGZQ4BHQO6TOZSB3R3JQ2ZRUP4A55WGPCYJF
- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
