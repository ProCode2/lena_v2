import init, { mount } from './pkg/magic.js';
async function run(ir) {
    await init();
    mount(ir);
}
let ir = {tag: 'App', value: '', info: { uses: ["./other.ln"],class: "this is my custom class",css: "display: flex;justify-content: center;flex-direction: column",id: "this is my custom id" },  children: [{tag: 'h1', value: '', info: { css: "text-align: center;background: grey;color: green" },  children: [{tag: 'text', value: 'can we parse the component above?', info: {  },  children: []}]},
{tag: 'Other', value: '', info: { class: "pretty_component",uses: ["./hello.ln"],css: "width: 100%;height: 100%;background: blue" },  children: [{tag: 'h1', value: '', info: {  },  children: [{tag: 'text', value: 'this is another component that i am using', info: {  },  children: []}]},
{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'Here is some more syntax to parse', info: {  },  children: []}]},
{tag: 'Hello', value: '', info: { class: "something" },  children: [{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'something else', info: {  },  children: []}]}]}]},
{tag: 'Other', value: '', info: { class: "pretty_component",uses: ["./hello.ln"],css: "width: 100%;height: 100%;background: blue" },  children: [{tag: 'h1', value: '', info: {  },  children: [{tag: 'text', value: 'this is another component that i am using', info: {  },  children: []}]},
{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'Here is some more syntax to parse', info: {  },  children: []}]},
{tag: 'Hello', value: '', info: { class: "something" },  children: [{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'something else', info: {  },  children: []}]}]}]}]};
run(ir);
