import init, { mount } from './pkg/magic.js';
async function run(ir) {
    await init();
    mount(ir);
}
let ir = {tag: 'App', value: '', info: { uses: ["./magic.ln"],css: "width: 100vw;height: 100vh;padding: 10px;background: #222;font-size: 16px;color: white" },  children: [{tag: 'h1', value: '', info: { css: "text-align: center" },  children: [{tag: 'text', value: 'this is the header of my app', info: {  },  children: []}]},
{tag: 'Magic', value: '', info: {  },  children: [{tag: 'div', value: '', info: { css: "display: flex;flex-direction: column;justify-content: center;align-items: center;color: red" },  children: [{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'This is a totally reusable component', info: {  },  children: []}]},
{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'believe me', info: {  },  children: []}]},
{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'I promise', info: {  },  children: []}]}]}]},
{tag: 'Magic', value: '', info: {  },  children: [{tag: 'div', value: '', info: { css: "display: flex;flex-direction: column;justify-content: center;align-items: center;color: red" },  children: [{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'This is a totally reusable component', info: {  },  children: []}]},
{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'believe me', info: {  },  children: []}]},
{tag: 'p', value: '', info: {  },  children: [{tag: 'text', value: 'I promise', info: {  },  children: []}]}]}]}]};
run(ir);
