# Lena: the Programming Language
Lena is a programming language that is currently under development. The goal is to simplify the given frontend development schenario and create something that is easy to use. It will focus towards removing some of the chaos of the frontend ecosystem by simply cutting down the huge dependency pipelines and build from scratch by implementing proven concepts like fine grained reactivity, Reusable components, Easy configurations and syntax.

## The current syntax:
```
// file: main.ln
App {
    class: "hello these are my custom classes",
    id: "uniqueId",

    h1 { "Hello this is a h1" }
    p { "Hello this is my {{hello}} app" }
}
```

## The CLI needs a `lena.toml` to find the lena files.
Example `lena.toml` file
```toml
entry = "./main.ln"
```

# Install and Run
- Clone the repository
- Run `cargo run --`
`Note: you need to have a *lena.toml* where you run the project`
