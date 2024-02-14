import init, { mount } from './pkg/magic.js';
async function run(ir) {
    await init();
    mount(ir);
}
let ir = {tag: 'App', value: '', info: { number: 123412,uses: ["./other.ln"],class: "this is my custom class",id: "this is my custom id" },  children: [{tag: 'h1', value: '', info: {  },  children: [{tag: 'text', value: 'can we parse the component above?', info: {  },  children: []}]},
{tag: 'Other', value: '', info: { class: "pretty_component",uses: ["./hello.ln"] },  children: [{tag: 'h1', value: '', info: {  },  children: [{tag: 'text', value: 'this is another component that i am using', info: {  },  children: []}]},
{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'Here is some more syntax to parse', info: {  },  children: []}]},
{tag: 'Hello', value: '', info: { class: "something" },  children: [{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'something else', info: {  },  children: []}]}]}]},
{tag: 'Other', value: '', info: { class: "pretty_component",uses: ["./hello.ln"] },  children: [{tag: 'h1', value: '', info: {  },  children: [{tag: 'text', value: 'this is another component that i am using', info: {  },  children: []}]},
{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'Here is some more syntax to parse', info: {  },  children: []}]},
{tag: 'Hello', value: '', info: { class: "something" },  children: [{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'something else', info: {  },  children: []}]}]}]}]};
run(ir);
