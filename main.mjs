import init, { v, update } from './pkg/veedoom.js';
import { render, onChange } from './dom.js';

window.veedoomOnChange = onChange;

async function main() {
  await init('/pkg/veedoom_bg.wasm');

  let example = v("div", {
    // key: "foo",
    text: "Hello... ",
    style: "font-size: 20px"
  }, []);

  // Initial render which sets up the root node
  render(example, "root");

  // Updates...
  setTimeout(() => {
    example = update(
      example,
      { ...example, tag: "code" }
    );
  }, 1000);

  setTimeout(() => {
    example = update(
      example,
      { ...example, props: {
        text: "HELLO... " 
      }}
    );
  }, 2000);

  setTimeout(() => {
    example = update(
      example,
      { ...example, children: [
        v("strong", {
          key: "bar",
          text: "VEEDOOM!",
          style: "color: red"
        }, [])
      ]}
    );
  }, 3000);

}
main()

