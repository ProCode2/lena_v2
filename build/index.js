function mount(rep){console.log("My IR is: ", rep)};
let ir = {tag: 'NO_TAG("App")', value: '', children: [{tag: 'NO_TAG("Other")', value: '', children: [{tag: 'H1', value: '', children: [{tag: 'TEXT', value: 'this is another component that i am using', children: []},
{tag: 'P', value: '', children: [{tag: 'TEXT', value: 'Here is some more syntax to parse', children: []},
{tag: 'NO_TAG("Hello")', value: '', children: [{tag: 'P', value: '', children: [{tag: 'TEXT', value: 'something else', children: []}]}]}]}]}]},
{tag: 'H1', value: '', children: [{tag: 'TEXT', value: 'can we parse the component above?', children: []}]}]};
mount(ir)