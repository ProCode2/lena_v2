import init, { mount } from "./pkg/magic.js";
async function run() {
  await init();
  const result = mount();
  console.log(`1 + 2 = ${result}`);
}
let ir = {
  tag: 'NO_TAG("App")',
  value: "",
  children: [
    {
      tag: 'NO_TAG("Other")',
      value: "",
      children: [
        {
          tag: "H1",
          value: "",
          children: [
            {
              tag: "TEXT",
              value: "this is another component that i am using",
              children: [],
            },
            {
              tag: "P",
              value: "",
              children: [
                {
                  tag: "TEXT",
                  value: "Here is some more syntax to parse",
                  children: [],
                },
                {
                  tag: 'NO_TAG("Hello")',
                  value: "",
                  children: [
                    {
                      tag: "P",
                      value: "",
                      children: [
                        { tag: "TEXT", value: "something else", children: [] },
                      ],
                    },
                  ],
                },
              ],
            },
          ],
        },
      ],
    },
    {
      tag: "H1",
      value: "",
      children: [
        {
          tag: "TEXT",
          value: "can we parse the component above?",
          children: [],
        },
      ],
    },
  ],
};
run();
