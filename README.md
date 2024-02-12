# Lena: the Programming Language
Lena is a programming language that is currently under development. The goal is to simplify the given frontend development schenario and create something that is easy to use. It will focus towards removing some of the chaos of the frontend ecosystem by simply cutting down the huge dependency pipelines and build from scratch by implementing proven concepts like fine grained reactivity, Reusable components, Easy configurations and syntax.

## Current Thought Process
I don't want to transpile to javascript or typescript. Instead compiling to WebAssembly seems very interesting route to take. However, I guess WASM does not have dom access, fetch API and all that yet. So The current route I am taking is this:
- Compile the lena code into an `Intermediate Representation(IR)`.
Currently I am just thinking of converting my [Component](https://github.com/ProCode2/lena_v2/blob/main/src/component.rs#L30) to a nested JS object.
```javascript
import wasmFunc from "somewhere";

let ir = {
    "tag": "App",
    "children": [
        {
            "tag": "H1",
            "children": [
                {
                    "tag": "TEXT",
                    "children": [],
                    "value": "Hello this is a h1",
                    "info": {}
                },
                {
                    "tag": "P",
                    "children": [
                        {
                            "tag": "TEXT",
                            "children": [],
                            "value": "Hello this is my {{hello}} app",
                            "info": {}
                        },
                        {
                            "tag": "DIV",
                            "children": [
                                {
                                    "tag": "TEXT",
                                    "children": [],
                                    "value": "Hello this is my new app",
                                    "info": {}
                                }
                            ],
                            "value": "",
                            "info": {}
                        }
                    ],
                    "value": "",
                    "info": {}
                }
            ],
            "value": "",
            "info": {}
        }
    ],
    "value": "",
    "info": {
        "id": "this is my custom id",
        "number": "123412",
        "uses": [
            "../other.ln"
        ],
        "class": "this is my custom class"
    }
}
wasmFunc(ir);
```
- Pass this intermediate representation to the wasm function compiled from rust that needs to be created which will then do the magic of converting the `IR` to DOM.

## The current syntax:
```
// file: main.ln
App {
    uses: [
        "../other.ln"
    ],
    class: "this is my custom class",
    id: "this is my custom id",
    number: 123412,

    h1 { "Hello this is a h1" }
    p { "Hello this is my {{hello}} app" }
    div { "Hello this is my new app" }
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

## TODO
- [x] Create a basic lexer
- [x] Create a basic parser to parse component body and info
- [x] parser should be able to also parse component that another component uses
- [ ] create Intermediate Representation
- [ ] Functionality to convert to and from IR
- [ ] Create the WASM module to generate the DOM from the IR
- [ ] State of the art error propagation and display
- [ ] Fine Grained Reactivity
