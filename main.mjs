import init, { create_element, render } from './pkg/veedoom.js';
async function main() {
  await init('/pkg/veedoom_bg.wasm');

  const example = create_element("div", {
    key: "foo",
    text: "Hello, "
  }, [
    create_element("strong", {
      key: "bar",
      text: "VEEDOOM!",
      style: "color: red"
    }, [])
  ]);

  render(example, "root");
}
main()
